use headless_chrome::{browser, logging, process, tab};
use headless_chrome::LaunchOptions;

#[test]
fn test_extension() -> Result<(), failure::Error> {
    let browser = browser::Browser::new(
        process::LaunchOptions{
            path: LaunchOptions::default_executable().unwrap(),
            headless: false,
            port: None,
            load_extension: Some(String::from("tests/extension_sampl"))
        }
    ).unwrap();
    // if there is popup like missing manifest.json
    // that could probably mean that extension didn't load successfully
    Ok(())
}