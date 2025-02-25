use std::sync::{Arc, Mutex};

use headless_chrome::Browser;
use headless_chrome::browser::tab::Tab;

mod server;

use anyhow::Result;

#[test]
fn expose_function() -> Result<()> {
    let server = server::Server::with_dumb_html(include_str!("simple.html"));

    let function_called_entries = Arc::new(Mutex::new(0));

    let browser = Browser::default()?;
    let tab: Arc<Tab> = browser.new_tab()?;

    let function_called_entries_clone = Arc::clone(&function_called_entries);

    let url = format!("http://127.0.0.1:{}", server.port());
    tab.navigate_to(&url)?.wait_until_navigated()?;

    tab.expose_function(
        "simple",
        Arc::new(move |_value| {
            *function_called_entries_clone.lock().unwrap() += 1;
        }),
    )?;

    tab.evaluate("window.simple('100')", false)?;

    assert_eq!(*function_called_entries.lock().unwrap(), 1);

    Ok(())
}
