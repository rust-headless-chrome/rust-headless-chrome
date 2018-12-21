#![feature(uniform_paths)]
#![feature(test)]
#![feature(duration_as_u128)]
#![feature(custom_attribute)]
#![allow(deprecated)]

extern crate log;

//use std::collections::HashMap;
//use std::io::Read;
//use std::process::{Command, Stdio, ChildStderr};
//use std::thread;
//use std::sync::Arc;
//use std::sync::Mutex;
//use std::env;
//
//use futures::sync::oneshot::Sender;
//use futures::Future;
//
//use regex::Regex;
//use websocket::{ClientBuilder, Message, OwnedMessage};
//use websocket::client::sync::Client;
//use websocket::stream::sync::TcpStream;
//
//use serde::de::{DeserializeOwned};
//use serde_json::Value;
//
//use cdp::{HasCdpCommand, SerializeCdpCommand};
//use cdp::browser::{GetVersionResponse, GetVersionCommand};
//use cdp::*;
//



pub mod errors;
pub mod evaluation;
pub mod chrome;
pub mod connection;
pub mod page_session;
pub mod waiting_call_registry;
pub mod cdtp;
//use chrome::{Chrome};
//use self::errors::{Result};


// path is from src/
//static COUNTER_HTML: &'static str = include_str!("../checkbox_counter.html");
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
//        let _ = super::it_works().unwrap();
//        println!("asdf");
    }
}