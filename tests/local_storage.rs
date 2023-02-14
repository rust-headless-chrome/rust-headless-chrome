use anyhow::Result;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use serde::{Deserialize, Serialize};

mod server;

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    pub value: i32,
}

#[test]
fn read_write_local_storage() -> Result<()> {
    let server = server::Server::with_dumb_html(include_str!("simple.html"));

    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.new_tab()?;

    let item_value = "cb2a8cd9";

    let url = format!("http://127.0.0.1:{}", server.port());
    tab.navigate_to(&url)?.wait_until_navigated()?;

    let value: String = tab.get_storage("translationHash")?;

    assert_ne!("", &value);

    tab.set_storage("translationHash", item_value)?;

    let new_value: String = tab.get_storage("translationHash")?;

    assert_eq!(item_value, new_value);

    tab.remove_storage("translationHash")?;

    assert!(tab.get_storage::<String>("translationHash").is_err());

    let item: Item = tab.get_storage("testItem")?;

    assert_eq!(300, item.value);

    Ok(())
}
