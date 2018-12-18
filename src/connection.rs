use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

use log::*;

use futures::sync::oneshot::Sender;
use futures::Future;

use websocket::{ClientBuilder, Message, OwnedMessage};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

use serde;
use serde_json::json;
use serde::de::DeserializeOwned;
use serde_json::Value;

use cdp::{HasCdpCommand, SerializeCdpCommand};

use super::errors::*;
use websocket::WebSocketError;

use super::chrome;

// this is duped in page session!!!
type CallId = u16;

type Response = Value;
type ResponseChannel = Sender<Response>;

pub struct Connection {
    sender: websocket::sender::Writer<TcpStream>,
    waiting_calls: Arc<Mutex<HashMap<CallId, ResponseChannel>>>,
    next_call_id: CallId,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodResponse {
    // TODO: should alias call IDs everywhere
    pub call_id: CallId,
    pub result: Value,
}

// this stuff should be in its own module b/c reused by page_session...
#[derive(Debug)]
pub struct Event {
    // TODO: could keep static const list of event names for sanity checking...
    name: String,
    params: Value,
}

#[derive(Debug)]
pub enum IncomingMessageKind {
    Event(Event),
    MethodResponse(MethodResponse),
}

// TODO: custom deserialize?!
// TODO: Message term overloaded in context of websockets?
enum IncomingMessage<T> {
    FromBrowser(T),
    FromTarget(T),
}

impl Connection {
    pub fn new(browser_id: &chrome::BrowserId, target_messages_tx: mpsc::Sender<MethodResponse>) -> Result<Self> {
        let connection = Connection::websocket_connection(&browser_id)?;

        let (websocket_receiver, sender) = connection.split().chain_err(|| "Couldn't split conn")?;

        let waiting_calls = Arc::new(Mutex::new(HashMap::new()));

        // TODO: can I clone inside spawn?
        let other_waiting_calls = Arc::clone(&waiting_calls);

        let _message_handling_thread = std::thread::spawn(move || {
            info!("starting msg handling loop");
            Self::handle_incoming_messages(websocket_receiver, &other_waiting_calls, target_messages_tx);
            info!("quit loop msg handling loop");
        });

        Ok(Connection {
            waiting_calls,
            sender,
            next_call_id: 0,
        })
    }

    // TODO: this method is too big
    fn handle_incoming_messages(mut receiver: websocket::receiver::Reader<TcpStream>,
                                waiting_calls: &Arc<Mutex<HashMap<CallId, ResponseChannel>>>,
                                target_messages_tx: mpsc::Sender<MethodResponse>) -> ()
    {
        trace!("Starting to handle messages");
        // TODO: ooh, use iterator magic to split events and method responses here?!
        for message in receiver.incoming_messages() {
            trace!("Received a message");

            match message {
                Err(error) => {
                    match error {
                        WebSocketError::NoDataAvailable => { return (); }
                        _ => { panic!("There was a problem opening the file: {:?}", error) }
                    }
                }
                Ok(OwnedMessage::Text(msg)) => {
                    trace!("Received text message: {:?}", msg);
                    let response: Response = serde_json::from_str(&msg).unwrap();

                    trace!("response = {:#?}", response);

                    let send_response_to_caller = |response: MethodResponse| {
                        trace!("response = {:#?}", response.call_id);
                        let mut waiting_calls_mut = waiting_calls.lock().unwrap();
                        trace!("locked waiting_calls");

                        let waiting_call_tx: ResponseChannel = waiting_calls_mut.remove(&response.call_id).unwrap();
                        waiting_call_tx.send(response.result).expect("failed to send response");
                    };

                    let parse_call_id = |response_id| {
                        match response_id {
                            Value::Number(num) => {
                                let error_msg = format!("Call ID is a serde number but can't be made into a CallId: {:?}", num);
                                Some(num.as_u64().expect(error_msg.as_ref()) as CallId)
                            }
                            Value::Null => {
                                None
                            }
                            _ => {
                                panic!("Weird response ID: not a number or null: {:?}", &response["id"])
                            }
                        }
                    };

                    let browser_call_id: Option<CallId> = parse_call_id(response["id"].clone());

                    let incoming_message: IncomingMessage<IncomingMessageKind> = match browser_call_id {
                        Some(call_id) => {
                            // TODO: gross
                            IncomingMessage::FromBrowser(IncomingMessageKind::MethodResponse(MethodResponse {
                                call_id: call_id,
                                result: response["result"].clone(),
                            }))
                        }
                        None => {
                            let params = &response["params"];
                            if let Value::String(response_string) = &params["message"] {
                                // TODO: DRY
                                let target_response: Response = serde_json::from_str(&response_string).unwrap();
                                dbg!(&target_response);
                                let target_call_id = parse_call_id(target_response["id"].clone());
                                IncomingMessage::FromTarget(IncomingMessageKind::MethodResponse(MethodResponse {
                                    call_id: target_call_id.expect("Response has message but not call id"),
                                    result: target_response["result"].clone(),
                                }))
                            } else {
                                IncomingMessage::FromTarget(IncomingMessageKind::Event(Event {
                                    name: response["method"].to_string(),
                                    params: response["params"].clone(),
                                }))
                                // TODO: it's an event from the target? not sure.
                            }
                        }
                    };

                    match incoming_message {
                        // TODO: huh, weird, we might not even need to distinguish these!
                        IncomingMessage::FromBrowser(msg) => {
                            match msg {
                                IncomingMessageKind::MethodResponse(response) => {
                                    // TODO: obviously overlap between response and message. result?
                                    send_response_to_caller(response);
                                }
                                IncomingMessageKind::Event(_) => {
                                    // TODO: this
                                }
                            }
                        }
                        IncomingMessage::FromTarget(msg) => {
                            match msg {
                                IncomingMessageKind::MethodResponse(response) => {
                                    // TODO: obviously overlap between response and message. result?
                                    target_messages_tx.send(response).expect("failed to send to page session");
                                }
                                IncomingMessageKind::Event(_ev) => {

                                    // TODO: this
                                }
                            }
                        }
                    }
                }
                _ => { warn!("Got a weird message..."); }
            }
        }
    }

    pub fn websocket_connection(browser_id: &chrome::BrowserId) -> Result<Client<TcpStream>> {
        let ws_url = &format!("ws://127.0.0.1:9223/devtools/browser/{}", browser_id);
        info!("Connecting to WebSocket: {}", ws_url);
        let client = ClientBuilder::new(ws_url)
            .chain_err(|| "Unable to create client builder")?
            .connect_insecure()
            .chain_err(|| "Unable to connect to WebSocket")?;

        info!("Successfully connected to WebSocket: {}", ws_url);

        Ok(client)
    }

    pub fn call_method<'a, R>(&mut self, command: &R::Command) -> Result<R>
        where R: DeserializeOwned + HasCdpCommand<'a>,
              <R as cdp::HasCdpCommand<'a>>::Command: serde::ser::Serialize + SerializeCdpCommand
    {
        trace!("Calling method");


        let method_id = self.next_call_id;
        self.next_call_id += 1;

        let method = json!({"method": command.command_name(), "id": method_id, "params": command});
        trace!("sending message: {:#?}", &method);
        let message = Message::text(serde_json::to_string(&method).unwrap());

        let (tx, rx) = futures::sync::oneshot::channel::<Response>();

        let my_clone = Arc::clone(&self.waiting_calls);

// putting this in its own scope to make sure the MutexGuard is dropped (and unlocked)
        {
            let mut waiting_calls = my_clone.lock().unwrap();
            waiting_calls.insert(method_id, tx);
        }

// what if this fails and the waiting method is left there forever?
        self.sender.send_message(&message).unwrap();

        let val = rx.map(|s| {
            serde_json::from_value::<R>(s.clone()).unwrap()
        }).wait().chain_err(|| "bad command")?;
        trace!("method caller got response");
        Ok(val as R)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    #[test]
    fn you_can_send_methods() {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true).unwrap();

        let (messages_tx, _messages_rx) = mpsc::channel::<super::MethodResponse>();

        let mut conn = super::Connection::new(&chrome.browser_id, messages_tx).unwrap();

        let _response1 = conn.call_method::<cdp::target::CreateBrowserContextResponse>(&cdp::target::CreateBrowserContextCommand {});
        let _response2 = conn.call_method::<cdp::target::GetBrowserContextsResponse>(&cdp::target::GetBrowserContextsCommand {}).unwrap();
        let response3 = conn.call_method::<cdp::target::GetTargetsResponse>(&cdp::target::GetTargetsCommand {}).unwrap();
        let first_target = &response3.target_infos[0];

        assert_eq!("about:blank", first_target.url);
    }
}
