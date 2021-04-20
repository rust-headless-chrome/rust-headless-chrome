use std::sync::Arc;

use failure::Fallible;

use headless_chrome::{Browser, LaunchOptions, Tab, browser::tab::SyncSendEvent, protocol::Event};

fn start() -> Fallible<()> {
    let browser = Browser::new(LaunchOptions {
        headless: false,
        ..Default::default()
    })?;

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to("https://www.google.com")
        .expect("failed to navigate");

    tab.wait_until_navigated().unwrap();

    let sync_event = SyncSendEvent(
        tab.clone(),
        Box::new(move |event: &Event, tab: &Tab| {
            match event {
                Event::Lifecycle(lifecycle) => {
                    if lifecycle.params.name == "DOMContentLoaded" {
                        
                        println!("{}",tab.get_url());
                    }
                }
                _ => {}
            }
        }),
    );

    tab.add_event_listener(Arc::new(sync_event)).unwrap();

    Ok(())
}

fn main() -> Fallible<()> {
    start()
}
