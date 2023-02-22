use std::env;

use headless_chrome::Browser;

use anyhow::Result;

#[test]
fn connect_to_url() -> Result<()> {
    // ignore in CI
    if std::env::var("RUST_CI").is_ok() {
        return Ok(())
    }

    let debug_ws_url = env::args().nth(1).expect("Must provide debug_ws_url");

    let browser = Browser::connect(debug_ws_url);

    assert!(browser.is_ok());

    Ok(())
}
