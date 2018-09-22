#[macro_use]
extern crate error_chain;

extern crate futures;

extern crate regex;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate websocket;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate cdp;

//
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio, Child, ChildStderr};
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use futures::sync::oneshot;
use futures::sync::oneshot::{Sender, Receiver};
use futures::Future;
use futures::executor;

use regex::Regex;
use websocket::{ClientBuilder, Message, WebSocketError, OwnedMessage};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
//
//use serde::Serializer;
use serde_json::Value;
//use websocket::message::OwnedMessage::Text;
//
use cdp::SerializeCdpCommand;
use cdp::target::{CreateTargetCommand, CreateTargetResponse};
//

mod errors;

use errors::*;

#[derive(Debug)]
struct BrowserId(String);

impl fmt::Display for BrowserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct Chrome {
    sender: websocket::sender::Writer<TcpStream>,
    waiting_calls: Arc<Mutex<HashMap<u32, Sender<Value>>>>,
}

impl Chrome {
    // TODO: find out why this complains if named 'new'
    fn build() -> Result<Self> {
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

        let (mut receiver, mut sender) = connection.split().chain_err(|| "Couldn't split conn")?;

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
                                waiting_calls: Arc<Mutex<HashMap<u32, Sender<Value>>>>) -> ()
    {
        trace!("Starting to handle messages");
        for message in receiver.incoming_messages() {
            trace!("Something happened");
            if let OwnedMessage::Text(msg) = message.unwrap() {
                trace!("Received message: {:?}", msg);
                let mut waiting_calls_mut = waiting_calls.lock().unwrap();
                let waiting_call_tx = waiting_calls_mut.remove(&1).unwrap();
                let response: Value = serde_json::from_str(&msg).unwrap();
                waiting_call_tx.send(response["result"].clone());
            } else {
                error!("Got a non text message?!")
            }
        }
    }

    fn call_method(&mut self) -> Receiver<Value> {
        trace!("Calling method");
        let my_clone = Arc::clone(&self.waiting_calls);
        let mut waiting_calls = my_clone.lock().unwrap();

        let (tx, rx) = futures::sync::oneshot::channel::<Value>();

        let method_id = 1;
        let method = json!({"method": "Browser.getVersion", "id":method_id, "params": {}});
        let message = Message::text(serde_json::to_string(&method).unwrap());

        waiting_calls.insert(method_id, tx);

        // what if this fails and the waiting method is left there forever?
        self.sender.send_message(&message).unwrap();

        rx
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

// we need to send a MethodCall, and we want to get back a Future representing the response
#[derive(Serialize, Deserialize, Debug)]
struct ReturnObject<T> {
    id: u32,
    result: T,
}

#[derive(Serialize, Deserialize, Debug)]
struct MethodCall<'a> {
    method: cdp::target::CreateTargetCommand<'a>,
    id: u32,
}

//
fn create_target_method(url: &str) -> MethodCall {
    let method = cdp::target::CreateTargetCommand {
        url: Cow::from(url),
        height: None,
        width: None,
        browser_context_id: None,
    };
    let id = 1;
    MethodCall { method, id }
}

//    let method_call = json!({"params": method, "method": method.method_name(), "id": 1});
//    eprintln!("serialized = {:#?}", method_call);
//    let message = Message::text(method_call.to_string());
//    if let Err(error) = self.client.send_message(&message) {
//        eprintln!("problem sending message! error: = {:#?}", error);
//    }
//
//    let response = match self.client.recv_message() {
//        Ok(msg) => match msg {
//            Text(ref msg_text) => {
//                eprintln!("msg_text = {:#?}", msg_text);
//                serde_json::from_str(msg_text).unwrap()
//            }
//            _ => panic!("received some weird thing")
//        }
//        Err(error) => panic!("problem recving message! error: = {:#?}", error)
//    };
//    eprintln!("response = {:#?}", response);
//    response
//        let response = self.call_method("Target.createTarget", seri);
//        serde_json::from_value(response).unwrap()


//


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        env_logger::init();
        let mut chrome = Chrome::build().expect("got chrome");

        chrome.call_method().map(|val| {
            eprintln!("val = {:#?}", val);
        }).wait();

//        rx.map(|blah| {
//
//        }).wait();


//        thread::spawn(move || {
//            // send this guy
//            for msg in receiver {
//                eprintln!("msg = {:#?}", msg);
//            }
//            let recvd_tx: futures::sync::oneshot::Sender<&str> = conn_rx.recv().unwrap();
//            recvd_tx.send("lol");
////            eprintln!("conn_rx.next() = {:#?}", recvd_tx);
//        });

//        conn_tx.send(tx);
//
//        rx.map(|response| {
//            eprintln!("method response = {:#?}", response);
//        }).wait();
//

//        assert_eq!(2, 2);
    }
}

//fn main() {
//    let mut chrome_process = chrome();
//
//    let browser_id = match chrome_process.stderr.as_mut() {
//        Some(ref mut stderr) => browser_id_from_output(stderr),
//        None => panic!("chrome didn't launch properly maybe?")
//    }.unwrap();
//
//    eprintln!("browser_id = {:#?}", browser_id);
//
//    let mut browser = Browser::connect(&browser_id).unwrap();
//
//    browser.create_target("https://google.com");
//
////    eprintln!("browser_version = {:#?}", browser.version()["result"]["product"]);
////
////    let target_id = browser.create_target("https://google.com").result.target_id;
////
////    let targets = browser.targets();
////
////    eprintln!("targets = {:#?}", browser.targets());
////
////    browser.call_method("Page.enable", json!({}));
////    let response = browser.call_method("Page.navigate", json!({"url": "https://wikipedia.org"}));
////    eprintln!("response = {:#?}", response);
////
//    browser.close();
////
////    // TODO: make this happen on drop?
////    if let Err(error) = chrome_process.kill() {
////        eprintln!("couldn't kill server! error = {:#?}", error);
////    }
//    // send a basic message like Browser.getVersion
//    // print return message
//}
//
//
//
//
//struct Browser {
//    client: Client<TcpStream>
//}
//
//// (this is what the devtools protocol calls it)
//#[derive(Serialize, Deserialize, Debug)]
//struct ReturnObject<T> {
//    id: u32,
//    result: T,
//}
//
//#[derive(Serialize, Deserialize)]
//#[serde(rename_all = "camelCase")]
//struct CreateTargetResult {
//    target_id: String
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//#[serde(rename_all = "camelCase")]
//struct TargetInfo {
//    attached: bool,
//    browser_context_id: String,
//    // seeing as we use this in multiple places, make it a struct?
//    target_id: String,
//    title: String,
//    #[serde(rename = "type")]
//    target_type: String,
//    url: String,
//}
//
//impl Browser {
//
//    fn call_method(&mut self, method: &str, params: Value) -> Value {
//        let json = json!({
//            "method": method,
//            "params": params,
//            "id": 1
//        });
//
//        eprintln!("data = {:#?}", json.to_string());
//        let message = Message::text(json.to_string());
//
//        if let Err(error) = self.client.send_message(&message) {
//            eprintln!("problem sending message! error: = {:#?}", error);
//        }
//
//        let response = match self.client.recv_message() {
//            Ok(msg) => match msg {
//                Text(ref msg_text) => serde_json::from_str(msg_text).unwrap(),
//                _ => panic!("received some weird thing")
//            }
//            Err(error) => panic!("problem recving message! error: = {:#?}", error)
//        };
//        eprintln!("response = {:#?}", response);
//        response
//    }
//
//    fn targets(&mut self) -> Vec<TargetInfo> {
//        // do I really need to clone here?
//        let response = self.call_method("Target.getTargets", json!({}))["result"]["targetInfos"].clone();
//        serde_json::from_value(response).unwrap()
//    }
//
//    //    fn target_info(&mut self, )
//    fn send_method(&mut self, method) {
//
//    }
//
//    //ReturnObject<CreateTargetResult>
//    fn create_target(&mut self, url: &str) -> ReturnObject<CreateTargetResponse> {
//        let method = cdp::target::CreateTargetmethod {
//            url: Cow::from(url),
//            height: None,
//            width: None,
//            browser_context_id: None,
//        };
//
//        let method_call = json!({"params": method, "method": method.method_name(), "id": 1});
//        eprintln!("serialized = {:#?}", method_call);
//        let message = Message::text(method_call.to_string());
//        if let Err(error) = self.client.send_message(&message) {
//            eprintln!("problem sending message! error: = {:#?}", error);
//        }
//
//        let response = match self.client.recv_message() {
//            Ok(msg) => match msg {
//                Text(ref msg_text) => {
//                    eprintln!("msg_text = {:#?}", msg_text);
//                    serde_json::from_str(msg_text).unwrap()
//                }
//                _ => panic!("received some weird thing")
//            }
//            Err(error) => panic!("problem recving message! error: = {:#?}", error)
//        };
//        eprintln!("response = {:#?}", response);
//        response
////        let response = self.call_method("Target.createTarget", seri);
////        serde_json::from_value(response).unwrap()
//    }
//
//    fn version(&mut self) -> Value {
//        self.call_method("Browser.getVersion", json!({}))
//    }
//
//    fn close(&mut self) -> Value {
//        self.call_method("Browser.close", json!({}))
//    }
//}
