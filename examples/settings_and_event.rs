use std::sync::Arc;

use anyhow::Result;

use headless_chrome::{protocol::cdp::types::Event, Browser, LaunchOptions};

fn start() -> Result<()> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to("https://www.google.com")
        .expect("failed to navigate");

    tab.wait_until_navigated().unwrap();

    let new_tab = tab.clone();

    let sync_event = Arc::new(move |event: &Event| match event {
        Event::PageLifecycleEvent(lifecycle) => {
            if lifecycle.params.name == "DOMContentLoaded" {
                println!("{}", new_tab.get_url());
            }
        }
        _ => {}
    });

    tab.add_event_listener(sync_event).unwrap();

    Ok(())
}

fn main() -> Result<()> {
    start()
}
