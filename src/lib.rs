#[macro_use]
extern crate error_chain;

extern crate futures;

extern crate regex;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate websocket;
//#[macro_use]
//extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate cdp;

//
//use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::process::{Command, Stdio, ChildStderr};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

use futures::sync::oneshot::Sender;
use futures::Future;

use regex::Regex;
use websocket::{ClientBuilder, Message, OwnedMessage};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

use serde::de::DeserializeOwned;
use serde_json::Value;

use cdp::{HasCdpCommand, SerializeCdpCommand};
use cdp::browser::{GetVersionResponse, GetVersionCommand};

use self::errors::*;

pub mod errors;


#[derive(Debug)]
struct BrowserId(String);

impl fmt::Display for BrowserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

type Response = Value;
type ResponseChannel = Sender<Response>;

struct Chrome {
    sender: websocket::sender::Writer<TcpStream>,
    waiting_calls: Arc<Mutex<HashMap<u32, ResponseChannel>>>,
}

impl Chrome {
    // TODO: find out why this complains if named 'new'
    fn new() -> Result<Self> {
        info!("Trying to start Chrome");

        let process = Command::new("/usr/bin/google-chrome")
            .args(&["--headless", "--remote-debugging-port=9393", "--verbose"])
            .stderr(Stdio::piped())
            .spawn()
            .chain_err(|| "Couldn't start chrome")?;

        info!("Started Chrome. PID: {}", process.id());

        let mut stderr = &mut process.stderr.unwrap();

        let browser_id = Chrome::browser_id_from_output(&mut stderr)
            .chain_err(|| "Couldn't get browser ID from Chrome process")?;
        let connection = Chrome::websocket_connection(browser_id)?;

        let (receiver, sender) = connection.split().chain_err(|| "Couldn't split conn")?;

        let waiting_calls = Arc::new(Mutex::new(HashMap::new()));

        let other_waiting_calls = Arc::clone(&waiting_calls);

        let _ = thread::spawn(move || {
            Chrome::handle_incoming_messages(receiver, other_waiting_calls);
        });

        return Ok(Chrome {
            waiting_calls,
            sender,
        });
    }

    fn handle_incoming_messages(mut receiver: websocket::receiver::Reader<TcpStream>,
                                waiting_calls: Arc<Mutex<HashMap<u32, ResponseChannel>>>) -> ()
    {
        trace!("Starting to handle messages");
        for message in receiver.incoming_messages() {
            trace!("Received a message");
            if let OwnedMessage::Text(msg) = message.unwrap() {
                trace!("Received text message: {:?}", msg);
                let mut waiting_calls_mut = waiting_calls.lock().unwrap();
                let waiting_call_tx: ResponseChannel = waiting_calls_mut.remove(&1).unwrap();
                let response: Response = serde_json::from_str(&msg).unwrap();
                let _ = waiting_call_tx.send(response);
            } else {
                error!("Got a non text message?!")
            }
        }
    }

    fn call_method<'a, R>(&mut self, command: R::Command) -> Box<Future<Item=R, Error=futures::Canceled>>
        where R: DeserializeOwned + HasCdpCommand<'a>,
              <R as cdp::HasCdpCommand<'a>>::Command: serde::ser::Serialize + SerializeCdpCommand
    {
        trace!("Calling method");
        let my_clone = Arc::clone(&self.waiting_calls);
        let mut waiting_calls = my_clone.lock().unwrap();

        let (tx, rx) = futures::sync::oneshot::channel::<Response>();

        let method_id = 1;
        let method = json!({"method": command.command_name(), "id":method_id, "params": command});

        let message = Message::text(serde_json::to_string(&method).unwrap());

        waiting_calls.insert(method_id, tx);

        // what if this fails and the waiting method is left there forever?
        self.sender.send_message(&message).unwrap();

        Box::new(rx.map(|s| {
            serde_json::from_value::<R>(s["result"].clone()).unwrap()
        }))
    }

    fn websocket_connection(browser_id: BrowserId) -> Result<Client<TcpStream>> {
        let ws_url = &format!("ws://127.0.0.1:9223/devtools/browser/{}", browser_id);
        info!("Connecting to WebSocket: {}", ws_url);
        let client = ClientBuilder::new(ws_url)
            .chain_err(|| "Unable to create client builder")?
            .connect_insecure()
            .chain_err(|| "Unable to connect to WebSocket")?;

        info!("Successfully connected to WebSocket: {}", ws_url);

        Ok(client)
    }


    fn browser_id_from_output(stderr: &mut ChildStderr) -> Result<BrowserId> {
        // TODO: user static or lazy static regex
        let re = Regex::new(r"listening on .*/devtools/browser/(.*)\n").unwrap();

        let extract = |text: &str| -> Option<String> {
            let caps = re.captures(text);
            let cap = &caps?[1];
            Some(cap.into())
        };

        let mut buf = [0; 200];
        // TODO: if can't find after a while, return error

        let time_before = std::time::SystemTime::now();
        loop {
            let elapsed_seconds = time_before
                .elapsed()
                .chain_err(|| "Couldn't get system time")?
                .as_secs();

            if elapsed_seconds > 1 {
                bail!("Couldn't read WebSocket URL within one second");
            }

            let bytes_read = stderr.read(&mut buf).unwrap();
            if bytes_read > 0 {
                let chrome_output = String::from_utf8_lossy(&buf);
                debug!("Chrome output: {}", chrome_output);

                match extract(&chrome_output) {
                    Some(browser_id) => return Ok(BrowserId(browser_id)),
                    None => continue
                };
            }
        }
    }
}


pub fn it_works() -> Result<()> {
    env_logger::init();
    let mut chrome = Chrome::new().expect("lol");
    let comm = GetVersionCommand {};
    chrome.call_method(comm)
        .map(|version: GetVersionResponse| {
            eprintln!("version = {:#?}", version.product);
        })
        .wait().chain_err(|| "oh boy")?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _ = super::it_works();
    }
}