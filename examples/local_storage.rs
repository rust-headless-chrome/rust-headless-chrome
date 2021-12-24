use anyhow::Result;

use headless_chrome::{Browser, LaunchOptions};

fn main() -> Result<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.wikipedia.com")
        .expect("failed to navigate");

    tab.wait_until_navigated().unwrap();

    let item: String = tab.get_storage("translationHash")?;

    println!("{}",item);

    Ok(())
}
