#![cfg_attr(feature = "nightly", feature(external_doc))]

extern crate log;
extern crate termcolor;

pub mod browser;
pub mod cdtp;
pub mod element;
pub mod helpers;
pub mod keys;
pub mod logging;
pub mod point;
pub mod process;
pub mod tab;
pub mod transport;
pub mod waiting_call_registry;
pub mod web_socket_connection;

pub use browser::{Browser, LaunchOptions};
pub use tab::Tab;

#[cfg(feature = "nightly")]
#[doc(include = "../README.md")]
#[allow(dead_code)]
type _READMETEST = ();
