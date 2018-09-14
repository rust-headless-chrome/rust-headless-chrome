extern crate websocket;
extern crate regex;
extern crate serde_json;

use std::fmt;
use std::io::Read;
use std::process::{Command, Stdio, Child, ChildStderr};
use regex::Regex;
use websocket::{ClientBuilder, Message};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::WebSocketError;

use serde_json::{Value, Error};
use websocket::message::OwnedMessage::Text;
#[derive(Debug)]
struct BrowserId(String);

impl fmt::Display for BrowserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn connect_to_remote_debugging_port(browser_id: BrowserId) -> Result<Client<TcpStream>, WebSocketError> {
    let ws_url = &format!("ws://127.0.0.1:9223/devtools/browser/{}", browser_id);
    eprintln!("ws_url = {}", ws_url);
    ClientBuilder::new(ws_url)
        .unwrap()
        .connect_insecure()
}

fn chrome() -> Child {
    Command::new("/usr/bin/google-chrome")
        .args(&["--headless", "--remote-debugging-port=9393", "--verbose"])
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn browser_id_from_output(stderr: &mut ChildStderr) -> Option<BrowserId> {
    // TODO: can this be extracted to a constant?
    let re = Regex::new(r"listening on .*/devtools/browser/(.*)\n").unwrap();

    let extract = |text: &str| -> Option<String> {
        let caps = re.captures(text);
        let cap = &caps?[1];
        Some(cap.into())
    };

    let mut buf = [0; 200];
    loop {
        let bytes_read = stderr.read(&mut buf).unwrap();
        if bytes_read > 0 {
            let chrome_output = String::from_utf8_lossy(&buf);
            eprintln!("chrome_output = {:#?}", chrome_output);

            // TODO: deal with the possibility of this loop never terminating!
            match extract(&chrome_output) {
                Some(browser_id) => return Some(BrowserId(browser_id)),
                None => continue
            };
        }
    }
}


fn main() {
    let mut chrome_process = chrome();

    let browser_id = match chrome_process.stderr.as_mut() {
        Some(ref mut stderr) => browser_id_from_output(stderr),
        None => panic!("chrome didn't launch properly maybe?")
    }.unwrap();

    eprintln!("browser_id = {:#?}", browser_id);

    match connect_to_remote_debugging_port(browser_id) {
        Ok(mut client) => {
            let data = r#"{"method": "Browser.getVersion","params": {}, "id":1}"#;

            eprintln!("data = {:#?}", data);
            let message = Message::text(data);

            if let Err(error) = client.send_message(&message) {
                eprintln!("problem sending message! error: = {:#?}", error);
            }

            match client.recv_message() {
                Ok(msg) => {
                    if let Text(ref msg_text) = msg {
                        let v: Value = serde_json::from_str(msg_text).unwrap();
                        if let Value::String(browser_version) = &v["result"]["product"] {
                            eprintln!("v = {:#?}", browser_version);
                        }
                    }
                }
                Err(error) => eprintln!("problem recving message! error: = {:#?}", error)
            }

            let data = r#"{"method": "Browser.close","params": {}, "id":2}"#;
            let message = Message::text(data);

            if let Err(error) = client.send_message(&message) {
                eprintln!("problem sending message! error: = {:#?}", error);
            }
            match client.recv_message() {
                Ok(msg) => eprintln!("closed browser"),
                Err(error) => eprintln!("problem recving message! error: = {:#?}", error)
            }
        }
        Err(error) => { eprintln!("error = {:#?}", error); }
    };

    // TODO: make this happen on drop?
    if let Err(error) = chrome_process.kill() {
        eprintln!("couldn't kill server! error = {:#?}", error);
    }
    // send a basic message like Browser.getVersion
    // print return message

}
