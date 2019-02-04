#![feature(uniform_paths)]
#![feature(test)]
#![feature(duration_as_u128)]
#![feature(custom_attribute)]
#![allow(deprecated)]

extern crate log;

pub mod chrome;
pub mod connection;
pub mod page_session;
pub mod waiting_call_registry;
pub mod point;
pub mod element;
pub mod tab;
pub mod keys;
pub mod cdtp;

