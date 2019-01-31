#![feature(uniform_paths)]
#![feature(test)]
#![feature(duration_as_u128)]
#![feature(custom_attribute)]
#![allow(deprecated)]

extern crate log;


pub mod errors;
pub mod evaluation;
pub mod chrome;
pub mod connection;
pub mod page_session;
pub mod waiting_call_registry;
pub mod point;
pub mod element;
pub mod tab;
pub mod cdtp;


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
