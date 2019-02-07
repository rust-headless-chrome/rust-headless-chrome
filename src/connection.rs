use std::sync::mpsc;

use failure::{Error};
use log::*;
use serde;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::WebSocketError;

use crate::cdtp;
use crate::cdtp::{Event, Response};
use crate::waiting_call_registry;

pub struct Connection {
    sender: websocket::sender::Writer<TcpStream>,
    call_registry: waiting_call_registry::WaitingCallRegistry,
}

impl Connection {
    pub fn new(ws_url: &str, target_messages_tx: mpsc::Sender<cdtp::Message>) -> Result<Self, Error> {
        let connection = Connection::websocket_connection(&ws_url)?;

        let (websocket_receiver, sender) = connection.split()?;

        let (browser_responses_tx, browser_responses_rx) = mpsc::channel();
        let call_registry = waiting_call_registry::WaitingCallRegistry::new(browser_responses_rx);

        let _message_handling_thread = std::thread::spawn(move || {
            trace!("Starting msg dispatching loop");
            Self::dispatch_incoming_messages(websocket_receiver, target_messages_tx, browser_responses_tx);
            trace!("Quit loop msg dispatching loop");
        });

        Ok(Connection {
            call_registry,
            sender,
        })
    }

    fn dispatch_incoming_messages(mut receiver: websocket::receiver::Reader<TcpStream>,
                                  target_messages_tx: mpsc::Sender<cdtp::Message>,
                                  browser_responses_tx: mpsc::Sender<Response>)
    {
        for ws_message in receiver.incoming_messages() {
            match ws_message {
                Err(error) => {
                    match error {
                        WebSocketError::NoDataAvailable => {}
                        _ => { panic!("Unhandled WebSocket error: {:?}", error) }
                    }
                }
                Ok(message) => {
                    if let OwnedMessage::Text(message_string) = message {
//                        trace!("Raw message: {:?}", message_string);
                        if let Ok(message) = cdtp::parse_raw_message(message_string) {
                            match message {
                                cdtp::Message::Response(response) => {
                                    browser_responses_tx.send(response).expect("failed to send to message to page session");
                                }

                                cdtp::Message::Event(event) => {
                                    match event {
                                        Event::ReceivedMessageFromTarget(target_message_event) => {
                                            let raw_message = target_message_event.params.message;
                                            if let Ok(target_message) = cdtp::parse_raw_message(raw_message.clone()) {
                                                target_messages_tx.send(target_message).expect("failed to send to page session");
                                            } else {
                                                trace!("Message from target isn't recognised: {:?}", &raw_message[..30]);
                                            }
                                        }
                                        _ => {
                                            trace!("Browser received event: {:?}", event);
                                        }
                                    }
                                }
                            }
                        } else {
                            debug!("Incoming message isn't recognised as event or method response");
                        }
                    } else {
                        panic!("Got a weird message: {:?}", message)
                    }
                }
            }
        }
    }

    pub fn websocket_connection(ws_url: &str) -> Result<Client<TcpStream>, Error> {
        // TODO: can't keep using that proxy forever, will need to deal with chromes on other ports
        let client = ClientBuilder::new(ws_url)?.connect_insecure()?;

        debug!("Successfully connected to WebSocket: {}", ws_url);

        Ok(client)
    }

    pub fn call_method<C>(&mut self, method: C) -> Result<C::ReturnObject, Error>
        where C: cdtp::Method + serde::Serialize {
        let call = method.to_method_call();
        let message = websocket::Message::text(serde_json::to_string(&call).unwrap());

        self.sender.send_message(&message).unwrap();

        let response_rx = self.call_registry.register_call(call.id);

        let response = response_rx.recv().unwrap();
        cdtp::parse_response::<C::ReturnObject>(response)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    #[test]
    fn you_can_send_methods() {
        env_logger::try_init().unwrap_or(());
        let chrome = crate::chrome::Chrome::new(Default::default()).unwrap();

        let (messages_tx, _messages_rx) = mpsc::channel::<crate::cdtp::Message>();

        let mut conn = crate::connection::Connection::new(&chrome.debug_ws_url, messages_tx).unwrap();

        let call = crate::cdtp::target::methods::CreateBrowserContext {};
        let r1 = conn.call_method(call).unwrap();

        dbg!(r1);
    }
}
