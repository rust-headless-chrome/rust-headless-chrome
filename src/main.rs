extern crate websocket;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate cdp;

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio, Child, ChildStderr};
use regex::Regex;
use websocket::{ClientBuilder, Message};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::WebSocketError;

use serde::Serializer;
use serde_json::Value;
use websocket::message::OwnedMessage::Text;

use cdp::SerializeCdpCommand;
use cdp::target::{CreateTargetResponse};

// TODO: parallel

// TODO: return Result<> here
fn main() {
    let mut chrome_process = chrome();

    let browser_id = match chrome_process.stderr.as_mut() {
        Some(ref mut stderr) => browser_id_from_output(stderr),
        None => panic!("chrome didn't launch properly maybe?")
    }.unwrap();

    eprintln!("browser_id = {:#?}", browser_id);

    let mut browser = Browser::connect(&browser_id).unwrap();

    browser.create_target("https://google.com");

//    eprintln!("browser_version = {:#?}", browser.version()["result"]["product"]);
//
//    let target_id = browser.create_target("https://google.com").result.target_id;
//
//    let targets = browser.targets();
//
//    eprintln!("targets = {:#?}", browser.targets());
//
//    browser.call_method("Page.enable", json!({}));
//    let response = browser.call_method("Page.navigate", json!({"url": "https://wikipedia.org"}));
//    eprintln!("response = {:#?}", response);
//
    browser.close();
//
//    // TODO: make this happen on drop?
//    if let Err(error) = chrome_process.kill() {
//        eprintln!("couldn't kill server! error = {:#?}", error);
//    }
    // send a basic message like Browser.getVersion
    // print return message
}

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
    // TODO: user static or lazy static regex
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

// (this is what the devtools protocol calls it)
#[derive(Serialize, Deserialize, Debug)]
struct ReturnObject<T> {
    id: u32,
    result: T,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateTargetResult {
    target_id: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TargetInfo {
    attached: bool,
    browser_context_id: String,
    // seeing as we use this in multiple places, make it a struct?
    target_id: String,
    title: String,
    #[serde(rename = "type")]
    target_type: String,
    url: String,
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

    fn call_method(&mut self, method: &str, params: Value) -> Value {
        let json = json!({
            "method": method,
            "params": params,
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
        eprintln!("response = {:#?}", response);
        response
    }

    fn targets(&mut self) -> Vec<TargetInfo> {
        // do I really need to clone here?
        let response = self.call_method("Target.getTargets", json!({}))["result"]["targetInfos"].clone();
        serde_json::from_value(response).unwrap()
    }

//    fn target_info(&mut self, )

    //ReturnObject<CreateTargetResult>
    fn create_target(&mut self, url: &str) -> ReturnObject<CreateTargetResponse> {
        let command = cdp::target::CreateTargetCommand {
            url: Cow::from(url),
            height: None,
            width: None,
            browser_context_id: None,
        };
        let method_call = json!({"params": command, "method": command.command_name(), "id": 1});
        eprintln!("serialized = {:#?}", method_call);
        let message = Message::text(method_call.to_string());
        if let Err(error) = self.client.send_message(&message) {
            eprintln!("problem sending message! error: = {:#?}", error);
        }
//match serde_json::from_str(msg_text) {
//                    Ok(val) => eprintln!("val = {:#?}", val),
//                    Err(err) => panic!(err)
//                },
        let response = match self.client.recv_message() {
            Ok(msg) => match msg {
                Text(ref msg_text) => {
                    eprintln!("msg_text = {:#?}", msg_text);
                    serde_json::from_str(msg_text).unwrap()
                },
                _ => panic!("received some weird thing")
            }
            Err(error) => panic!("problem recving message! error: = {:#?}", error)
        };
        eprintln!("response = {:#?}", response);
        response
//        let response = self.call_method("Target.createTarget", seri);
//        serde_json::from_value(response).unwrap()
    }

    fn version(&mut self) -> Value {
        self.call_method("Browser.getVersion", json!({}))
    }

    fn close(&mut self) -> Value {
        self.call_method("Browser.close", json!({}))
    }
}
