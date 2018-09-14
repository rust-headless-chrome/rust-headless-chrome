extern crate websocket;
extern crate regex;

use std::io::Read;
use std::process::Command;
//use websocket::{ClientBuilder, Message};
use std::process::{Stdio, Child, ChildStderr};
use regex::Regex;

#[derive(Debug)]
struct BrowserId(String);

fn browser_id_from_output(stderr: &mut ChildStderr) -> Option<BrowserId> {
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
            // TODO: surely this can get stuck waiting forever for chrome to give us the output?
            match extract(&chrome_output) {
                Some(browser_id) => return Some(BrowserId(browser_id)),
                None => continue
            };
        }
    }
}


fn chrome () -> Child {
     Command::new("/usr/bin/google-chrome")
        .args(&["--headless", "--remote-debugging-port=9393"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn main() {
    let mut chrome_process = chrome();

    let browser_id = match chrome_process.stderr.as_mut() {
        Some(ref mut stderr) => browser_id_from_output(stderr),
        None => panic!("chrome didn't launch properly maybe?")
    };

    eprintln!("browser_id = {:#?}", browser_id.unwrap());

    chrome_process.wait().unwrap();
}
