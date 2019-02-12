#![feature(uniform_paths)]
#![feature(test)]
#![feature(duration_as_u128)]
#![feature(custom_attribute)]
#![allow(deprecated)]

extern crate log;
extern crate termcolor;

pub mod point;
pub mod helpers;
pub mod element;
pub mod keys;
pub mod logging;
pub mod cdtp;
pub mod browser;
pub mod process;
pub mod web_socket_connection;
pub mod waiting_call_registry;
pub mod tab;
pub mod transport;
