use std::fs::File;
use std::io::prelude::*;

use failure::Error;
use log::*;
use toml;

use lib::chrome;

fn parse_secrets() -> Result<toml::Value, Error> {
    let mut secrets_toml = File::open("./secrets.toml")?;
    let mut secrets = String::new();
    secrets_toml.read_to_string(&mut secrets).unwrap();

    Ok(secrets.parse::<toml::Value>()?)
}

fn log_in_to_ml() -> Result<(), Error> {
    env_logger::try_init().unwrap_or(());

    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: true, ..Default::default() })?;
    let tab = chrome.new_tab()?;

    if let Err(nav_failed) = tab.navigate_to("https://app-staging.mentorloop.com/") {
        warn!("Mentorloop seems to be down.");
        return Ok(());
    }

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

    std::thread::sleep_ms(2000);

    let secrets = &parse_secrets()?["fastmail"];

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Tab")?;
    tab.type_str(secrets["password"].as_str().unwrap())?;
    tab.press_key("Enter")?;

    tab.wait_until_navigated()?;

    tab.find_element(".icon-compose")?.click()?;

    tab.find_element(".s-compose-to ")?.click()?;

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Enter")?; // for the autocomplete
    tab.press_key("Tab")?;
    tab.type_str("A test subject line!")?;
    tab.press_key("Tab")?;
    tab.type_str("Test body")?;

    tab.find_element("button.s-send")?.click()?;

    std::thread::sleep_ms(5000);

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
