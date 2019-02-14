# Headless Chrome
[![Crate](https://img.shields.io/crates/v/headless_chrome.svg)](https://crates.io/crates/headless_chrome)
[![API](https://docs.rs/headless_chrome/badge.svg)](https://docs.rs/headless_chrome)

[Puppeteer](https://github.com/GoogleChrome/puppeteer) for Rust. It looks a little something like this:

```rust
use headless_chrome::browser::{Browser, LaunchOptions};

let browser = Browser::new(LaunchOptions {  
    headless: true,  
    path: "/usr/bin/google-chrome", // BYO Chrome binary
   ..Default::default()  
})?;  

let tab = browser.wait_for_initial_tab()?;
	
tab.navigate_to("https://www.wikipedia.org")?;  
  
tab.wait_for_element(r#"input#searchInput"#)?.click()?;  
  
tab.type_str("WebKit")?;  
tab.press_key("Enter")?;  
  
tab.wait_for_element("#firstHeading")?;  
  
assert_eq!(true, tab.get_url().ends_with("WebKit"));
```

For fuller examples, take a look at `tests/integration.rs`.

## Missing features
* Documentation
* Frame / iframe support
* `window.alert` handlers
## Contributing
Pull requests and issues are most welcome, even if they're just experience reports. If you find anything frustrating or confusing, let me know!

