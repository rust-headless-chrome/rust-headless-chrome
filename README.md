# Headless Chrome
[![Build Status](https://travis-ci.com/atroche/rust-headless-chrome.svg?branch=master)](https://travis-ci.com/atroche/rust-headless-chrome)
[![Crate](https://img.shields.io/crates/v/headless_chrome.svg)](https://crates.io/crates/headless_chrome)
[![API](https://docs.rs/headless_chrome/badge.svg)](https://docs.rs/headless_chrome)

[Puppeteer](https://github.com/GoogleChrome/puppeteer) for Rust. It looks a little something like this:

```rust
use headless_chrome::{Browser, LaunchOptionsBuilder};

fn browse_wikipedia() -> Result<(), failure::Error> {
    let options = LaunchOptionsBuilder::default().unwrap().expect("Failed to find chrome");
    let browser = Browser::new(options)?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;

    tab.wait_for_element("input#searchInput")?
       .click()?;
    tab.type_str("WebKit")?
       .press_key("Enter")?;

    tab.wait_for_element("#firstHeading")?;
    assert_eq!(true, tab.get_url().ends_with("WebKit"));
    Ok(())
}

assert!(browse_wikipedia().is_ok());
```

For fuller examples, take a look at [`tests/simple.rs`](tests/simple.rs) and [`examples/real_world.rs`](examples/real_world.rs).

## Missing features
* Documentation
* Frame / iframe support
* `window.alert` handlers
## Contributing
Pull requests and issues are most welcome, even if they're just experience reports. If you find anything frustrating or confusing, let me know!

