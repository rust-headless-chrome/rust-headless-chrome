use std::fs::File;
use std::io::prelude::*;

use failure::Error;
use log::*;
use toml;

use env_logger;
use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use rand::distributions::Alphanumeric;
use rand::{self, Rng};
use std::sync::Arc;

fn main() {
    env_logger::init();
    wikipedia();
    fastmail();
    digital_pigeon();
    dropbox();
}

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

fn default_browser_and_tab() -> (Browser, Arc<Tab>) {
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options).expect("Failed to launch and connect to Chrome");
    let tab = browser
        .wait_for_initial_tab()
        .expect("Problem finding Chrome's initial tab");
    (browser, tab)
}

fn browse_wikipedia() -> Result<(), Error> {
    let (_browser, tab) = default_browser_and_tab();

    tab.navigate_to("https://www.wikipedia.org")?;

    tab.wait_for_element(r#"input#searchInput"#)?.click()?;

    tab.type_str("WebKit")?;
    tab.press_key("Enter")?;

    tab.wait_for_element("#firstHeading")?;

    assert_eq!(true, tab.get_url().ends_with("WebKit"));

    Ok(())
}

fn log_in_to_digital_pigeon() -> Result<(), Error> {
    let (browser, tab) = default_browser_and_tab();

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

    tab.wait_for_element(".create-new-item-btn")?.click()?;

    // warning: there are two li.add-dropbox elements on the page
    sleep(2000);
    let add_via_dropbox_button = tab.wait_for_element(".popover li.add-dropbox")?;
    add_via_dropbox_button.click()?;

    let dropbox_tab = wait_for(
        || {
            let tabs_mutex = browser.get_tabs();
            let tabs = tabs_mutex.lock().unwrap();
            if tabs.len() > 1 {
                Some(Arc::clone(&tabs.last().unwrap()))
            } else {
                None
            }
        },
        WaitOptions {
            timeout_ms: 1000,
            sleep_ms: 100,
        },
    )?;

    log_in_to_dropbox(&dropbox_tab)?;

    // for pre-captcha
    wait_until_true(
        || {
            dropbox_tab
                .get_url()
                .starts_with("https://www.dropbox.com/chooser")
        },
        WaitOptions {
            timeout_ms: 120_000,
            sleep_ms: 100,
        },
    )?;

    dropbox_tab
        .wait_for_element(".dropins-search-input")?
        .click()
        .unwrap();
    dropbox_tab.type_str("digital")?;
    dropbox_tab.wait_until_navigated()?;

    let movie_row =
        dropbox_tab.wait_for_element(".dropins-chooser-files-list-item .mc-checkbox")?;
    movie_row.click()?;
    dropbox_tab
        .wait_for_element(".mc-button-primary")?
        .click()
        .unwrap();

    tab.wait_for_element_with_custom_timeout(
        ".file.status-completed",
        std::time::Duration::from_secs(40),
    )?;

    Ok(())
}

fn log_in_to_fastmail_and_send_email() -> Result<(), Error> {
    let (_browser, tab) = default_browser_and_tab();

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

fn log_in_to_dropbox(tab: &Tab) -> Result<(), Error> {
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

    Ok(())
}
//

fn wikipedia() {
    browse_wikipedia().expect("passed");
}

fn fastmail() {
    log_in_to_fastmail_and_send_email().expect("passed");
}

fn digital_pigeon() {
    log_in_to_digital_pigeon().expect("passed");
}

fn dropbox() {
    let (_browser, tab) = default_browser_and_tab();

    let nav_result = tab.navigate_to("https://www.dropbox.com/login");
    assert_eq!(true, nav_result.is_ok());

    log_in_to_dropbox(&tab).expect("passed");
}

pub fn wait_until_true<F>(predicate: F, wait_options: WaitOptions) -> Result<(), Error>
where
    F: Fn() -> bool,
{
    wait_for(
        || {
            if predicate() {
                Some(())
            } else {
                None
            }
        },
        wait_options,
    )
}

use failure::Fail;
use std::time::{Duration, SystemTime};

#[derive(Debug, Fail)]
#[fail(display = "The thing you were waiting for never came")]
pub struct TimedOut {}

pub struct WaitOptions {
    pub timeout_ms: u64,
    pub sleep_ms: u64,
}

pub fn wait_for<F, G>(predicate: F, wait_options: WaitOptions) -> Result<G, Error>
where
    F: Fn() -> Option<G>,
{
    let time_before = SystemTime::now();
    loop {
        let elapsed = time_before.elapsed()?;

        if elapsed > Duration::from_millis(wait_options.timeout_ms) {
            return Err(TimedOut {}.into());
        }

        if let Some(thing) = predicate() {
            return Ok(thing);
        }

        std::thread::sleep(std::time::Duration::from_millis(wait_options.sleep_ms));
    }
}
