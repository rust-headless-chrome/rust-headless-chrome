extern crate websocket;
extern crate regex;

use std::io::Read;
use std::process::Command;
//use websocket::{ClientBuilder, Message};
use std::process::{Stdio, ChildStderr};
use regex::Regex;

fn oh_boy(stderr: &mut ChildStderr) -> Option<()> {
    let re = Regex::new(r"listening on .*/devtools/browser/(.*)\n").unwrap();

    // TODO: closure regex

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
            let browser_id = match extract(&chrome_output) {
                Some(browser_id) => browser_id,
                None => continue
            };

            eprintln!("browser_id = {:#?}", browser_id);
        }
    }
}

fn main() {
    let mut chrome_process = Command::new("/usr/bin/google-chrome")
        .args(&["--headless", "--remote-debugging-port=9393"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let y = &mut chrome_process;

    // can read from this badboy now
    match y.stderr {
        Some(ref mut stderr) => oh_boy(stderr),
        None => panic!("asdf")
    };

    y.wait().unwrap();
}
