extern crate websocket;
extern crate regex;
#[macro_use]
extern crate serde_json;

use std::fmt;
use std::io::Read;
use std::process::{Command, Stdio, Child, ChildStderr};
use regex::Regex;
use websocket::{ClientBuilder, Message};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::WebSocketError;

use serde_json::{Value};
use websocket::message::OwnedMessage::Text;

#[derive(Debug)]
struct BrowserId(String);

impl fmt::Display for BrowserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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

struct Browser {
    client: Client<TcpStream>
}

impl Browser {
    fn connect(id: &BrowserId) -> Result<Browser, WebSocketError> {
        let ws_url = &format!("ws://127.0.0.1:9223/devtools/browser/{}", id);
        eprintln!("ws_url = {}", ws_url);
        let client = ClientBuilder::new(ws_url)
            .unwrap()
            .connect_insecure()?;

        Ok(Browser { client })
    }

    fn call_method(&mut self, method: &str) -> Value {
        let json = json!({
            "method": method,
            "params": {},
            "id": 1
        });

        eprintln!("data = {:#?}", json.to_string());
        let message = Message::text(json.to_string());

        if let Err(error) = self.client.send_message(&message) {
            eprintln!("problem sending message! error: = {:#?}", error);
        }

        let response = match self.client.recv_message() {
            Ok(msg) => match msg {
                Text(ref msg_text) => serde_json::from_str(msg_text).unwrap(),
                _ => panic!("received some weird thing")
            }
            Err(error) => panic!("problem recving message! error: = {:#?}", error)
        };
        response
    }

    fn version(&mut self) -> Value {
        self.call_method("Browser.getVersion")
    }

    fn close(&mut self) -> Value {
        self.call_method("Browser.close")
    }
}

fn main() {
    let mut chrome_process = chrome();

    let browser_id = match chrome_process.stderr.as_mut() {
        Some(ref mut stderr) => browser_id_from_output(stderr),
        None => panic!("chrome didn't launch properly maybe?")
    }.unwrap();

    eprintln!("browser_id = {:#?}", browser_id);

    let mut browser = Browser::connect(&browser_id).unwrap();

    eprintln!("browser_version = {:#?}", browser.version()["result"]["product"]);

    browser.close();

    // TODO: make this happen on drop?
    if let Err(error) = chrome_process.kill() {
        eprintln!("couldn't kill server! error = {:#?}", error);
    }
    // send a basic message like Browser.getVersion
    // print return message
}
