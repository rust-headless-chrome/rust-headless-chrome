#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

extern crate serde;
extern crate regex;
extern crate websocket;
extern crate futures;

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

use serde::de::{DeserializeOwned};
use serde_json::Value;

extern crate cdp;
use cdp::{HasCdpCommand, SerializeCdpCommand};
use cdp::browser::{GetVersionResponse, GetVersionCommand};
use cdp::*;


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
    waiting_calls: Arc<Mutex<HashMap<u64, ResponseChannel>>>,
    next_call_id: u64,
}

impl Chrome {
    // TODO: find out why this complains if named 'new'
    fn new() -> Result<Self> {
        info!("Trying to start Chrome");

//        let process = Command::new("/usr/bin/google-chrome")
        let process = Command::new("/home/alistair/Downloads/chrome-linux/chrome")
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
            Chrome::handle_incoming_messages(receiver, &other_waiting_calls);
            trace!("quit loop msg handling loop");
        });

        Ok(Chrome {
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
            if let OwnedMessage::Text(msg) = message.unwrap() {
                trace!("Received text message: {:?}", msg);
                let response: Response = serde_json::from_str(&msg).unwrap();

                eprintln!("response = {:#?}", response);

                let response_id: u64 = match &response["id"] {
                    Value::Number(num) => num.as_u64().unwrap(),
                    // indicates they sent an event rather than a method response. ignore for now.
                    Value::Null => { eprintln!("null = "); continue; },
                    _ => panic!("bad response ID")
                };
                eprintln!("response = {:#?}", response["id"]);
                let mut waiting_calls_mut = waiting_calls.lock().unwrap();
                trace!("locked waiting_calls");

                let waiting_call_tx: ResponseChannel = waiting_calls_mut.remove(&response_id).unwrap();
                let _ = waiting_call_tx.send(response);
                trace!("Passed response back to waiting method");
            } else {
                error!("Got a non text message?!")
            }
        }
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

        let mut buf = [0; 512];

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

    // TODO: find a way of making this return a thing which doesn't need type annotations
    fn call_method<'a, R>(&mut self, command: &R::Command) -> Result<R>
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
        Ok(val)
    }
}

// path is from src/
static COUNTER_HTML: &'static str = include_str!("../checkbox_counter.html");

pub fn it_works() -> Result<()> {
    env_logger::init();
    let chrome = &mut Chrome::new()?;

    let comm = browser::GetVersionCommand {};
    let response: GetVersionResponse = chrome.call_method(&comm)?;

    let response = chrome.call_method::<target::GetTargetsResponse>(&target::GetTargetsCommand {})?;
    let default_target = &response.target_infos[0];

    let response: target::AttachToTargetResponse = chrome.call_method(&target::AttachToTargetCommand {
        target_id: default_target.target_id.clone(),
        flatten: Some(false)
    })?;

    let comm = &page::EnableCommand {};
    let method = json!({"method": comm.command_name(), "id":9999, "params": comm});
    let message_str = serde_json::to_string(&method).unwrap();
    trace!("sending message: {:#?}", &message_str);

    let session_id = response.session_id;

    let response: target::SendMessageToTargetResponse = chrome.call_method(&target::SendMessageToTargetCommand {
        message: std::borrow::Cow::Borrowed(&message_str),
        target_id: Some(default_target.target_id.clone()),
        session_id: Some(session_id.clone()),
    })?;

//    let response: page::EnableResponse = chrome.call_method(&page::EnableCommand {})?;
//
    let comm = page::NavigateCommand {
        url: std::borrow::Cow::Borrowed("https://wikipedia.org"),
        referrer: None,
        transition_type: None,
        frame_id: None
    };
    let method = json!({"method": comm.command_name(), "id":99999, "params": comm});
    let message_str = serde_json::to_string(&method).unwrap();
    let response: target::SendMessageToTargetResponse = chrome.call_method(&target::SendMessageToTargetCommand {
        message: std::borrow::Cow::Borrowed(&message_str),
        target_id: Some(default_target.target_id.clone()),
        session_id: Some(session_id.clone()),
    })?;
//
//    thread::sleep(std::time::Duration::from_secs(1));
//
//    let response: page::GetFrameTreeResponse = chrome.call_method(&page::GetFrameTreeCommand {})?;
//    let main_frame_id = response.frame_tree.frame.id;
//
//    let response: page::SetDocumentContentResponse = chrome.call_method(&page::SetDocumentContentCommand {
//        frame_id: main_frame_id.clone(),
//        html: std::borrow::Cow::Borrowed(COUNTER_HTML)
//    })?;

//    eprintln!("target_id = {:#?}", default_target.target_id);
//        .and_then(|version: GetVersionResponse| {
//            eprintln!("version = {:#?}", version.product);
//
//            chrome.call_method(&comm).and_then(|version: GetVersionResponse| {
//                eprintln!("version = {:#?}", version.product);
//                Ok(())
//            }).wait();
//            Ok(())
//        })
//        .wait().chain_err(|| "oh boy")?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let _ = super::it_works().unwrap();
        println!("asdf");
    }
}