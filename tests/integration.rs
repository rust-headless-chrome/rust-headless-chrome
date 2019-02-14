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
use lib::helpers::wait_for;
use lib::helpers::WaitOptions;
use std::sync::Arc;
use lib::helpers::wait_until_true;

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
        headless: false,
        path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        ..Default::default()
    })?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;

    tab.wait_for_element(r#"input#searchInput"#)?.click()?;

    tab.type_str("WebKit")?;
    tab.press_key("Enter")?;

    tab.wait_for_element("#firstHeading")?;

    assert_eq!(true, tab.get_url().ends_with("WebKit"));
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
fn log_in_to_digital_pigeon() -> Result<(), Error> {
    logging::enable_logging();

    let browser = browser::Browser::new(browser::LaunchOptions {
        path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        headless: false,
        ..Default::default()
    })?;

    let tab = browser.wait_for_initial_tab()?;

    // so we can use it to upload files
//    log_in_to_dropbox(&tab);

    if let Err(_nav_failed) = tab.navigate_to("https://www.digitalpigeon.com/login") {
        warn!("Digital Pigeon seems to be down.");
        return Ok(());
    }

    let secrets = &parse_secrets()?["digital_pigeon"];


    let element = tab.wait_for_element(r#"input[type="email"]"#)?;
    let classic_mp = element.get_midpoint()?;
    let js_mp = element.get_js_midpoint()?;

    assert_eq!(classic_mp, js_mp);

    element.click()?;

    tab.type_str(secrets["email"].as_str().unwrap())?;
    tab.press_key("Enter")?;

    tab.wait_for_element("input#password")?.click()?;

    tab.type_str(secrets["password"].as_str().unwrap())?;
    tab.press_key("Enter")?;

    tab.wait_until_navigated()?;

    // TODO: be able to wait for elements to become visible
    tab.wait_for_element(".create-new-item-btn")?.click()?;

    // warning: there are two li.add-dropbox elements on the page
    sleep(2000);
    let add_via_dropbox_button = tab.wait_for_element(".popover li.add-dropbox")?;
    add_via_dropbox_button.click()?;

    let dropbox_tab = wait_for(|| {
        let tabs_mutex = browser.get_tabs();
        let tabs = tabs_mutex.lock().unwrap();
        if tabs.len() > 1 {
            Some(Arc::clone(&tabs.last().unwrap()))
        } else {
            None
        }
    }, WaitOptions {
        timeout_ms: 1000,
        sleep_ms: 100
    })?;

    log_in_to_dropbox(&dropbox_tab)?;

    // for pre-captcha
    wait_until_true(|| {
        dropbox_tab.get_url().starts_with("https://www.dropbox.com/chooser")
    }, WaitOptions {
        timeout_ms: 120_000,
        sleep_ms: 100
    })?;

    dropbox_tab.wait_for_element(".dropins-search-input")?.click();
    dropbox_tab.type_str("digital")?;
    dropbox_tab.wait_until_navigated()?;

    let movie_row = dropbox_tab.wait_for_element(".dropins-chooser-files-list-item .mc-checkbox")?;
    movie_row.click()?;
    dropbox_tab.wait_for_element(".mc-button-primary")?.click();

    tab.wait_for_element_with_custom_timeout(".file.status-completed", 40_000)?;

    Ok(())
}
//
fn log_in_to_fastmail() -> Result<(), Error> {
    logging::enable_logging();

    let browser = browser::Browser::new(browser::LaunchOptions {
        path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
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
    // NOTE: must be already on the dropbox login page
    let secrets = &parse_secrets()?["dropbox"];

    let email_field = tab.wait_for_element(r#"input[type="email"]"#)?;
    email_field.type_into(&secrets["email"].as_str().unwrap())?;
    sleep(100);

    tab.press_key("Tab")?;
    sleep(100);
    tab.type_str(secrets["password"].as_str().unwrap())?;
    sleep(100);

    tab.press_key("Enter")?;
    sleep(100);

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

#[test]
fn digital_pigeon() {
//    for i in 0..30 {
//        println!("ATTEMPT NUM: {}", i);
        log_in_to_digital_pigeon().expect("passed");
//    }
}

#[test]
fn dropbox() {
    logging::enable_logging();

    let browser = browser::Browser::new(browser::LaunchOptions {
        path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        headless: true,
        ..Default::default()
    }).unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();

    let nav_result = tab.navigate_to("https://www.dropbox.com/login");
    assert_eq!(true, nav_result.is_ok());

    log_in_to_dropbox(&tab).expect("passed");
}
