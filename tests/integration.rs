extern crate headless_chrome;
use headless_chrome;

#[test]
fn it_adds_two() {
    let chrome = headless_chrome::chrome::Chrome::new(true).unwrap();

    // TODO: test you can make two sessions from one chrome thing!
    // inspect headfully at first!

    let mut session = headless_chrome::page_session::PageSession::new(&chrome.browser_id).unwrap();
    assert_eq!(4, 2 + 2);
}