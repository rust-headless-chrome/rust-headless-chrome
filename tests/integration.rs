use failure::{Error};
use log::*;

use lib::chrome;

#[test]
fn it_does_basic_browser_tests() {
//    let chrome = lib::chrome::Chrome::new(true).unwrap();
//
//    let mut tab = lib::page_session::PageSession::new(&chrome.browser_id).unwrap();
//    // TODO: test you can make two sessions from one chrome thing!
//    // inspect headfully at first!
//
//    // TODO: chrome.new_tab()
//
//    tab.navigate_to("http://todomvc.com/examples/vanillajs/");

//    let get_item_count_text = || { tab.find_element(".todo-count").text() };
//
//    tab.type_string("Buy an adjustable spanner");
//    tab.press_key(lib::keyboard::Enter);
//
//    assert_eq!("1 item left", get_item_count_text());
//
//    tab.find_element("input.toggle").click();
//
//    assert_eq!("0 items left", get_item_count_text());
}

fn log_in_to_ml() -> Result<(), Error> {
    env_logger::try_init().unwrap_or(());
    let chrome = chrome::Chrome::new(chrome::LaunchOptions { headless: false })?;
    let tab = chrome.new_tab()?;

    tab.navigate_to("https://app-staging.mentorloop.com/")?;
    std::thread::sleep_ms(3000);

    let element = tab.find_element(r#"input[type="email"]"#)?;

    dbg!(element.get_attributes());
//    dbg!(element.get_attributes());
//    tab.type_str("roche.a@gmail.com")?;
//    tab.press_key("Enter")?;sdf
    std::thread::sleep_ms(10000);

    Ok(())
}

#[test]
fn ml_staging() {
    log_in_to_ml().expect("passed");
}
