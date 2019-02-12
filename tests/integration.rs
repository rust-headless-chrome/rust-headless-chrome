use std::fs::File;
use std::io::prelude::*;

use failure::Error;
use log::*;
use toml;

use lib;
use lib::logging;
use lib::browser;
use rand::{self, Rng};
use rand::distributions::Alphanumeric;

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn parse_secrets() -> Result<toml::Value, Error> {
    let mut secrets_toml = File::open("./secrets.toml")?;
    let mut secrets = String::new();
    secrets_toml.read_to_string(&mut secrets).unwrap();

    Ok(secrets.parse::<toml::Value>()?)
}

fn rand_ascii() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect()
}

fn browse_wikipedia() -> Result<(), Error> {
    logging::enable_logging();

    let browser = browser::Browser::new(browser::LaunchOptions {
        headless: true,
        ..Default::default()
    })?;
    let tab = browser.wait_for_initial_tab()?;

    if let Err(_nav_failed) = tab.navigate_to("https://www.wikipedia.org") {
        warn!("Site seems to be down.");
        return Ok(());
    }


    let log_in_link = tab.wait_for_element(r#"#js-link-box-en"#)?;

    log_in_link.click()?;

//    tab.wait_until_navigated()?;
//    sleep(1000);

    Ok(())
}

//
//fn log_in_to_ml() -> Result<(), Error> {
//    logging::enable_logging();
//
//    let chrome = chrome::Process::new(chrome::LaunchOptions { headless: true, ..Default::default() })?;
//    let tab = chrome.new_tab()?;
//
//    if let Err(_nav_failed) = tab.navigate_to("https://app-staging.mentorloop.com/") {
//        warn!("Mentorloop seems to be down.");
//        return Ok(());
//    }
//
//    let _element = tab.find_element(r#"input[type="email"]"#)?;
//
//    tab.type_str("roche.a@gmail.com")?;
//    tab.press_key("Enter")?;
//
//    Ok(())
//}
//
//
//
//fn log_in_to_digital_pigeon() -> Result<(), Error> {
//    logging::enable_logging();
//
//    let chrome = chrome::Process::new(chrome::LaunchOptions { headless: true, ..Default::default() })?;

//
//    let tab = chrome.new_tab()?;
//
//    // so we can use it to upload files
////    log_in_to_dropbox(&tab);
//
//    if let Err(_nav_failed) = tab.navigate_to("https://www.digitalpigeon.com/login") {
//        warn!("Digital Pigeon seems to be down.");
//        return Ok(());
//    }
//
//    let secrets = &parse_secrets()?["digital_pigeon"];
//
//
//    let element = tab.wait_for_element(r#"input[type="email"]"#)?;
//    let classic_mp = element.get_midpoint()?;
//    let js_mp = element.get_js_midpoint()?;
//
//    assert_eq!(classic_mp, js_mp);
//
//    element.click()?;
//
//    tab.type_str(secrets["email"].as_str().unwrap())?;
//    tab.press_key("Enter")?;
//
//    tab.wait_for_element("input#password")?.click()?;
//
//    tab.type_str(secrets["password"].as_str().unwrap())?;
//    tab.press_key("Enter")?;
//    tab.wait_until_navigated()?;
//
//    // TODO: be able to wait for elements to become visible
//    tab.wait_for_element(".create-new-item-btn")?.click()?;
//
//    // warning: there are two li.add-dropbox elements on the page
//    tab.wait_for_element(".popover li.add-dropbox")?.click()?;
//
//    // TODO: handle this:
//    // 🏹  [16:58:52.342] - connection   - Message from target isn't recognised: "{\"method\":\"Page.windowOpen\",\"p"
//
////    tab.wait_for_element("li.add-dropbox .needsclick")?.click()?;
////    tab.wait_for_element(".icon-dropbox")?.click()?;
////    tab.wait_for_element("div.popover")?.click()?;
//
//
//    sleep(100_000);
////
////    let element = tab.wait_for_element(r#"input[type="file"]"#)?;
////    element.set_input_files(&vec!["/tmp/blah"])?;
////
////    let element = tab.wait_for_element(r#"input[directory=""]"#)?;
////    element.set_input_files(&vec!["/tmp/blah"])?;
//
//    Ok(())
//}
//
fn log_in_to_fastmail() -> Result<(), Error> {
    logging::enable_logging();

    let browser = browser::Browser::new(browser::LaunchOptions {
        headless: false,
        ..Default::default()
    })?;
    let tab = browser.wait_for_initial_tab()?;

    if let Err(_nav_failed) = tab.navigate_to("https://www.fastmail.com/login") {
        warn!("Fastmail seems to be down.");
        return Ok(());
    }

    let secrets = &parse_secrets()?["fastmail"];

    let _email_field = tab.wait_for_element(r"input.v-Text-input")?;

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Tab")?;
    tab.type_str(secrets["password"].as_str().unwrap())?;
    tab.press_key("Enter")?;

    tab.wait_for_element(".icon-compose")?.click()?;

    tab.wait_for_element(".s-compose-to ")?.click()?;

    let subject = rand_ascii();
    let body = rand_ascii();

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Enter")?; // for the autocomplete
    tab.press_key("Tab")?;

    tab.type_str(&subject)?;
    tab.press_key("Tab")?;

    tab.type_str(&body)?;

    tab.wait_for_element("button.s-send")?.click()?;

    // refresh inbox:
    tab.wait_for_element("li.v-MailboxSource--inbox")?.click()?;

    Ok(())
}

fn log_in_to_dropbox(tab: &lib::tab::Tab) -> Result<(), Error> {
    if let Err(_nav_failed) = tab.navigate_to("https://www.dropbox.com/login") {
        warn!("Dropbox seems to be down.");
        return Ok(());
    }

    let secrets = &parse_secrets()?["dropbox"];

    let email_field = tab.wait_for_element(r#"input[type="email"]"#)?;
    email_field.type_into(&secrets["email"].as_str().unwrap())?;

    tab.press_key("Tab")?;
    tab.type_str(secrets["password"].as_str().unwrap())?;

    tab.press_key("Enter")?;

//    tab.wait_until_navigated()?;

//    tab.wait_for_element("a#files")?.click()?;
//
//    tab.wait_until_navigated()?;

    Ok(())
}
//
#[test]
fn wikipedia() {
    browse_wikipedia().expect("passed");
}
//
#[test]
fn fastmail() {
    log_in_to_fastmail().expect("passed");
}
//
//#[test]
//fn ml_staging() {
//    log_in_to_ml().expect("passed");
//}
//
//#[test]
//fn digital_pigeon() {
//    log_in_to_digital_pigeon().expect("passed");
//}
//
#[test]
fn dropbox() {
    logging::enable_logging();

    let browser = browser::Browser::new(browser::LaunchOptions {
        headless: true,
        ..Default::default()
    }).unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    log_in_to_dropbox(&tab).expect("passed");
}
