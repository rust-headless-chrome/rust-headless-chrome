#![feature(uniform_paths)]
#![feature(test)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

use std::collections::HashMap;
use std::io::Read;
use std::process::{Command, Stdio, ChildStderr};
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::env;

use futures::sync::oneshot::Sender;
use futures::Future;

use regex::Regex;
use websocket::{ClientBuilder, Message, OwnedMessage};
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

use serde::de::{DeserializeOwned};
use serde_json::Value;

use cdp::{HasCdpCommand, SerializeCdpCommand};
use cdp::browser::{GetVersionResponse, GetVersionCommand};
use cdp::*;

use error_chain::bail;



pub mod errors;
pub mod evaluation;
pub mod chrome;
use chrome::{Chrome};
use self::errors::{Result};


// path is from src/
static COUNTER_HTML: &'static str = include_str!("../checkbox_counter.html");

pub fn it_works() -> Result<()> {
    env_logger::init();
    let chrome = &mut Chrome::new()?;

    let comm = GetVersionCommand {};
    let _response: GetVersionResponse = chrome.call_method(&comm)?;

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

    let _response: target::SendMessageToTargetResponse = chrome.call_method(&target::SendMessageToTargetCommand {
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
    let _response: target::SendMessageToTargetResponse = chrome.call_method(&target::SendMessageToTargetCommand {
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
//        let _ = super::it_works().unwrap();
//        println!("asdf");
    }
}