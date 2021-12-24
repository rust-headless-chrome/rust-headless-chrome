use std::ffi::OsStr;

use anyhow::Result;

use headless_chrome::{browser::default_executable, Browser, LaunchOptions};

#[test]
fn test_extension() -> Result<()> {
    Browser::new(
        LaunchOptions::default_builder()
            .path(Some(default_executable().unwrap()))
            .extensions(vec![OsStr::new("tests/extension_sampl")])
            .build()
            .unwrap(),
    )
    .unwrap();
    // if there is popup like missing manifest.json
    // that could probably mean that extension didn't load successfully
    Ok(())
}
