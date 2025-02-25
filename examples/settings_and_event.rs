use std::sync::Arc;

use anyhow::Result;

use headless_chrome::{Browser, LaunchOptions, protocol::cdp::types::Event};

fn start() -> Result<()> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;

    let tab = browser.new_tab().unwrap();

    tab.navigate_to("https://www.google.com")
        .expect("failed to navigate");

    tab.wait_until_navigated().unwrap();

    let new_tab = tab.clone();

    let sync_event = Arc::new(move |event: &Event| {
        if let Event::PageLifecycleEvent(lifecycle) = event {
            if lifecycle.params.name == "DOMContentLoaded" {
                println!("{}", new_tab.get_url());
            }
        }
    });

    tab.add_event_listener(sync_event).unwrap();

    Ok(())
}

fn main() -> Result<()> {
    start()
}
