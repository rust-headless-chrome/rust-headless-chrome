use std::sync::Arc;

use failure::Fallible;

use headless_chrome::{protocol::Event, Browser, LaunchOptions};

fn start() -> Fallible<()> {
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
        Event::Lifecycle(lifecycle) => {
            if lifecycle.params.name == "DOMContentLoaded" {
                println!("{}", new_tab.get_url());
            }
        }
        _ => {}
    });

    tab.add_event_listener(sync_event).unwrap();

    Ok(())
}

fn main() -> Fallible<()> {
    start()
}
