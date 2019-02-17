extern crate log;
extern crate termcolor;

pub mod browser;
pub mod cdtp;
mod element;
mod helpers;
mod keys;
mod point;
mod process;
mod tab;
mod transport;
mod waiting_call_registry;
mod web_socket_connection;

#[cfg(test)]
mod logging;

pub use browser::{Browser, LaunchOptions};
