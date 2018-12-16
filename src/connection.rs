use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

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

type Response = Value;
type ResponseChannel = Sender<Response>;

pub struct Connection {
    sender: websocket::sender::Writer<TcpStream>,
    waiting_calls: Arc<Mutex<HashMap<u64, ResponseChannel>>>,
    next_call_id: u64,
}

impl Connection {
    pub fn new(browser_id: &chrome::BrowserId) -> Result<Self> {
        let connection = Connection::websocket_connection(&browser_id)?;

        let (receiver, sender) = connection.split().chain_err(|| "Couldn't split conn")?;

        let waiting_calls = Arc::new(Mutex::new(HashMap::new()));

        let other_waiting_calls = Arc::clone(&waiting_calls);

        let _message_handling_thread = std::thread::spawn(move || {
            info!("starting msg handling loop");
            Connection::handle_incoming_messages(receiver, &other_waiting_calls);
            info!("quit loop msg handling loop");
        });

        Ok(Connection {
            waiting_calls,
            sender,
            next_call_id: 0,
        })
    }

    fn handle_incoming_messages(mut receiver: websocket::receiver::Reader<TcpStream>,
                                waiting_calls: &Arc<Mutex<HashMap<u64, ResponseChannel>>>) -> ()
    {
        trace!("Starting to handle messages");
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

                    let response_id: u64 = match &response["id"] {
                        Value::Number(num) => num.as_u64().unwrap(),
                        // indicates they sent an event rather than a method response. ignore for now.
                        Value::Null => {
                            eprintln!("null = ");
                            continue;
                        }
                        _ => panic!("bad response ID")
                    };
                    trace!("response = {:#?}", response["id"]);
                    let mut waiting_calls_mut = waiting_calls.lock().unwrap();
                    trace!("locked waiting_calls");

                    let waiting_call_tx: ResponseChannel = waiting_calls_mut.remove(&response_id).unwrap();
                    let _ = waiting_call_tx.send(response);
                    trace!("Passed response back to waiting method");
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

    // TODO: shouldn't this really consume command?
    pub fn call_method<'a, R>(&mut self, command: &R::Command) -> Result<R>
        where R: DeserializeOwned + HasCdpCommand<'a>,
              <R as cdp::HasCdpCommand<'a>>::Command: serde::ser::Serialize + SerializeCdpCommand
    {
        trace!("Calling method");

        let (tx, rx) = futures::sync::oneshot::channel::<Response>();

        let my_clone = Arc::clone(&self.waiting_calls);

        let method_id = self.next_call_id;
        self.next_call_id += 1;

        let method = json!({"method": command.command_name(), "id":method_id, "params": command});
        trace!("sending message: {:#?}", &method);
        let message = Message::text(serde_json::to_string(&method).unwrap());

        // putting this in its own scope to make sure the MutexGuard is dropped (and unlocked)
        {
            let mut waiting_calls = my_clone.lock().unwrap();
            waiting_calls.insert(method_id, tx);
        }

        // what if this fails and the waiting method is left there forever?
        self.sender.send_message(&message).unwrap();

        let val = rx.map(|s| {
            serde_json::from_value::<R>(s["result"].clone()).unwrap()
        }).wait().chain_err(|| "bad command")?;
        trace!("method caller got response");
        Ok(val as R)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn you_can_send_methods() {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true).unwrap();

        let mut conn = super::Connection::new(&chrome.browser_id).unwrap();

        let response1 = conn.call_method::<cdp::target::CreateBrowserContextResponse>(&cdp::target::CreateBrowserContextCommand {});
        let response2 = conn.call_method::<cdp::target::GetBrowserContextsResponse>(&cdp::target::GetBrowserContextsCommand {}).unwrap();
        let response3 = conn.call_method::<cdp::target::GetTargetsResponse>(&cdp::target::GetTargetsCommand {}).unwrap();
        let first_target = &response3.target_infos[0];
        assert_eq!("about:blank", first_target.url);
    }
}
