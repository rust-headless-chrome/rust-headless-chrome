use std::env;

use headless_chrome::Browser;

use anyhow::Result;

#[test]
fn connect_to_url() -> Result<()> {

    let debug_ws_url = env::args().nth(1).expect("Must provide debug_ws_url");

    let browser = Browser::connect(debug_ws_url.to_string());

    assert!(browser.is_ok());

    Ok(())
}
