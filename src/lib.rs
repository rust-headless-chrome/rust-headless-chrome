#![cfg_attr(feature = "nightly", feature(external_doc))]

extern crate log;
extern crate termcolor;

mod browser;
pub mod cdtp;

pub use browser::{Browser, LaunchOptions, Tab};

#[cfg(feature = "nightly")]
#[doc(include = "../README.md")]
#[allow(dead_code)]
type _READMETEST = ();
