use std::fs::File;
use std::io::prelude::*;

use failure::Error;
use log::*;
use toml;

use lib::chrome;
use lib::logging;
use rand::{self, Rng};
use rand::distributions::Alphanumeric;

fn parse_secrets() -> Result<toml::Value, Error> {
    let mut secrets_toml = File::open("./secrets.toml")?;
    let mut secrets = String::new();
    secrets_toml.read_to_string(&mut secrets).unwrap();

    Ok(secrets.parse::<toml::Value>()?)
}

fn log_in_to_ml() -> Result<(), Error> {
    logging::enable_logging();

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

fn rand_ascii() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect()
}



fn log_in_to_digital_pigeon() -> Result<(), Error> {
    logging::enable_logging();
    let time_before = std::time::SystemTime::now();

    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: false, ..Default::default() })?;
    let tab = chrome.new_tab()?;

    if let Err(nav_failed) = tab.navigate_to("https://www.digitalpigeon.com/login") {
        warn!("Digital Pigeon seems to be down.");
        return Ok(());
    }

    let secrets = &parse_secrets()?["digital_pigeon"];

    dbg!(secrets);

    let element = tab.find_element(r#"input[type="email"]"#)?;
    element.click()?;

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Enter")?;

    tab.wait_for_element("input#password")?.click()?;

    tab.type_str(secrets["password"].as_str().unwrap())?;
    tab.press_key("Enter")?;
    tab.wait_until_navigated()?;

    tab.wait_for_element("button.close")?.click()?;

    tab.wait_for_element(".create-new-item-btn")?.click()?;

//    std::thread::sleep_ms(10000);
    // TODO: if you can't compute quads via protocol, try via JS runtime and getBoundingClientRect
    tab.wait_for_element("div")?.click()?;
//
    let element = tab.wait_for_element(r#"input[type="file"]"#)?;
    element.set_input_files(&vec!["/tmp/blah"])?;

    let element = tab.wait_for_element(r#"input[directory=""]"#)?;
    element.set_input_files(&vec!["/tmp/blah"])?;

    std::thread::sleep_ms(1000000);

    Ok(())
}

fn log_in_to_fastmail() -> Result<(), Error> {
    logging::enable_logging();
    let time_before = std::time::SystemTime::now();

    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: true, ..Default::default() })?;
    let tab = chrome.new_tab()?;

    if let Err(nav_failed) = tab.navigate_to("https://www.fastmail.com/login") {
        warn!("Fastmail seems to be down.");
        return Ok(());
    }

    let secrets = &parse_secrets()?["fastmail"];

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Tab")?;
    tab.type_str(secrets["password"].as_str().unwrap())?;
    tab.press_key("Enter")?;

    tab.wait_until_navigated()?;

    tab.find_element(".icon-compose")?.click()?;

    tab.find_element(".s-compose-to ")?.click()?;

    let subject = rand_ascii();
    let body = rand_ascii();

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Enter")?; // for the autocomplete
    tab.press_key("Tab")?;

    tab.type_str(&subject)?;
    tab.press_key("Tab")?;

    tab.type_str(&body)?;

    tab.find_element("button.s-send")?.click()?;

    let elapsed_seconds = time_before
        .elapsed()?
        .as_secs();

    dbg!(elapsed_seconds);


    std::thread::sleep_ms(100);

    // refresh inbox:
    tab.find_element("li.v-MailboxSource--inbox")?.click();

    std::thread::sleep_ms(5000);

    Ok(())
}

fn browse_wikipedia() -> Result<(), Error> {
    logging::enable_logging();
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
#[test]
fn digital_pigeon() {
    log_in_to_digital_pigeon().expect("passed");
}
