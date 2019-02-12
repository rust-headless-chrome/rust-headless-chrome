use std::sync::mpsc;

use failure::Error;
use log::*;
use serde;
use websocket::{ClientBuilder, OwnedMessage};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::WebSocketError;

use crate::cdtp;
use crate::cdtp::{Event, Response};
use std::sync::Mutex;

pub struct WebSocketConnection {
    sender: Mutex<websocket::sender::Writer<TcpStream>>,
}

impl WebSocketConnection {
    pub fn new(ws_url: &str, target_messages_tx: mpsc::Sender<cdtp::Message>) -> Result<Self, Error> {
        let connection = WebSocketConnection::websocket_connection(&ws_url)?;
        let (websocket_receiver, sender) = connection.split()?;

        std::thread::spawn(move || {
            trace!("Starting msg dispatching loop");
            Self::dispatch_incoming_messages(websocket_receiver, target_messages_tx);
            trace!("Quit loop msg dispatching loop");
        });

        Ok(WebSocketConnection {
            sender: Mutex::new(sender),
        })
    }

    fn dispatch_incoming_messages(mut receiver: websocket::receiver::Reader<TcpStream>,
                                  messages_tx: mpsc::Sender<cdtp::Message>)
    {
        for ws_message in receiver.incoming_messages() {
            match ws_message {
                Err(error) => {
                    match error {
                        WebSocketError::NoDataAvailable => {}
                        WebSocketError::IoError(err) => {
                            warn!("{}", err);
                            break;
                        }
                        _ => { panic!("Unhandled WebSocket error: {:?}", error) }
                    }
                }
                Ok(message) => {
                    if let OwnedMessage::Text(message_string) = message {
                        if let Ok(message) = cdtp::parse_raw_message(&message_string) {
                            if messages_tx.send(message).is_err() {
                                break;
                            }
                        } else {
                            debug!("Incoming message isn't recognised as event or method response: {}", message_string);
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

    pub fn send_message(&self, message_text: &str) -> Result<(), Error> {
        let message = websocket::Message::text(message_text);
        let mut sender = self.sender.lock().unwrap();
        sender.send_message(&message)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use crate::logging;

    #[test]
    fn you_can_send_messages() {
        logging::enable_logging();
        let chrome = crate::browser::Browser::new(Default::default()).unwrap();

        let (messages_tx, _messages_rx) = mpsc::channel::<crate::cdtp::Message>();

        let mut conn = super::WebSocketConnection::new(&chrome.process.debug_ws_url, messages_tx).unwrap();

        let call = crate::cdtp::target::methods::CreateBrowserContext {};
        let r1 = conn.send_message("hello").unwrap();
    }
}
