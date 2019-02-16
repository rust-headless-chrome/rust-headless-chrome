extern crate log;
extern crate termcolor;

pub mod browser;
pub mod cdtp;
mod tab;
mod point;
mod process;
mod transport;
mod waiting_call_registry;
mod web_socket_connection;
mod element;
mod helpers;
mod keys;

#[cfg(test)]
mod logging;

pub use browser::{Browser, LaunchOptions};
