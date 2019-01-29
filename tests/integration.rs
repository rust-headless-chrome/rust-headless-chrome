#[test]
fn it_does_basic_browser_tests() {
    let chrome = lib::chrome::Chrome::new(true).unwrap();

    let mut tab = lib::page_session::PageSession::new(&chrome.browser_id).unwrap();
    // TODO: test you can make two sessions from one chrome thing!
    // inspect headfully at first!

    // TODO: chrome.new_tab()

    tab.navigate_to("http://todomvc.com/examples/vanillajs/");

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