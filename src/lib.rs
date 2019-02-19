#![cfg_attr(feature = "nightly", feature(external_doc))]

extern crate log;

#[macro_use]
extern crate derive_builder;

mod browser;
pub mod cdtp;

pub use browser::{Browser, LaunchOptionsBuilder, Tab};

#[cfg(feature = "nightly")]
#[doc(include = "../README.md")]
#[allow(dead_code)]
type _READMETEST = ();
