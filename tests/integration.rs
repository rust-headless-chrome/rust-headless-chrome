use failure::{Error};
use log::*;

use lib::chrome;

fn log_in_to_ml() -> Result<(), Error> {
    env_logger::try_init().unwrap_or(());
    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: true, ..Default::default() })?;
    let tab = chrome.new_tab()?;

    if let Err(nav_failed) = tab.navigate_to("https://app-staging.mentorloop.com/") {
        warn!("Mentorloop seems to be down.");
        return Ok(());
    }
//    std::thread::sleep_ms(3000);

    let element = tab.find_element(r#"input[type="email"]"#)?;

    dbg!(element.get_attributes());
    tab.type_str("roche.a@gmail.com")?;
    tab.press_key("Enter")?;

    Ok(())
}

fn log_in_to_fastmail() -> Result<(), Error> {
    env_logger::try_init().unwrap_or(());
    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: false, ..Default::default() })?;
    let tab = chrome.new_tab()?;

    if let Err(nav_failed) = tab.navigate_to("https://www.fastmail.com/") {
        warn!("Fastmail seems to be down.");
        return Ok(());
    }

    let log_in_link = tab.find_element(r#"#header-login a"#)?;
    dbg!(log_in_link.get_description());

    log_in_link.click()?;

    tab.wait_until_navigated()?;

    tab.type_str("alistair@fastmail.com");
    tab.press_key("Tab");
    tab.type_str("password");
    tab.press_key("Enter");

    Ok(())
}

fn browse_wikipedia() -> Result<(), Error> {
    env_logger::try_init().unwrap_or(());
    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: false, ..Default::default() })?;
    let tab = chrome.new_tab()?;

    if let Err(nav_failed) = tab.navigate_to("https://www.wikipedia.org") {
        warn!("Site seems to be down.");
        return Ok(());
    }

    let log_in_link = tab.find_element(r#"#js-link-box-en"#)?;
    dbg!(log_in_link.get_description());

    log_in_link.click()?;

    tab.wait_until_navigated()?;

    Ok(())
}

#[test]
fn wikipedia() {
    browse_wikipedia().expect("passed");
}
#[test]
fn fastmail() {
    log_in_to_fastmail().expect("passed");
}
#[test]
fn ml_staging() {
    log_in_to_ml().expect("passed");
}
