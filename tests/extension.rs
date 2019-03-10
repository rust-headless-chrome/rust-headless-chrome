use headless_chrome::{browser::default_executable, Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;

#[test]
fn test_extension() -> Result<(), failure::Error> {
    Browser::new(
        LaunchOptionsBuilder::default()
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
