# Headless Chrome

[![Build Status](https://travis-ci.com/atroche/rust-headless-chrome.svg?branch=master)](https://travis-ci.com/atroche/rust-headless-chrome)
[![Crate](https://img.shields.io/crates/v/headless_chrome.svg)](https://crates.io/crates/headless_chrome)
[![API](https://docs.rs/headless_chrome/badge.svg)](https://docs.rs/headless_chrome)
[![Discord channel](https://img.shields.io/discord/557374784233799681.svg?logo=discord)](https://discord.gg/yyGEzcc)

[Puppeteer](https://github.com/GoogleChrome/puppeteer) for Rust. It looks a little something like this:

```rust
use headless_chrome::{Browser, protocol::page::ScreenshotFormat};

fn browse_wikipedia() -> Result<(), failure::Error> {
    let browser = Browser::default()?;

    let tab = browser.wait_for_initial_tab()?;

    /// Navigate to wikipedia
    tab.navigate_to("https://www.wikipedia.org")?;

    /// Wait for network/javascript/dom to make the search-box available
    /// and click it.
    tab.wait_for_element("input#searchInput")?.click()?;

    /// Type in a query and press `Enter`
    tab.type_str("WebKit")?.press_key("Enter")?;

    /// We should end up on the WebKit-page once navigated
    tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));

    /// Take a screenshot of the entire browser window
    let _jpeg_data = tab.capture_screenshot(
                        ScreenshotFormat::JPEG(Some(75)),
                        None,
                        true)?;

    /// Take a screenshot of just the WebKit-Infobox
    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    Ok(())
}

assert!(browse_wikipedia().is_ok());
```

For fuller examples, take a look at [`tests/simple.rs`](tests/simple.rs) and [`examples/real_world.rs`](examples/real_world.rs).

If you're looking to do general browser testing or scraping (rather than anything specific to Chrome / DevTools), you're probably better off with [fantoccini](https://github.com/jonhoo/fantoccini) for now. It's a lot more feature-complete and stable.

## Version numbers

Starting with v0.2.0, we're trying to follow SemVar strictly.

## Troubleshooting

If you get errors related to timeouts, you likely need to enable sandboxing either in the kernel or as a setuid sandbox. Puppeteer has some information about how to do that [here](https://github.com/GoogleChrome/puppeteer/blob/master/docs/troubleshooting.md)

By default, `headless_chrome` will download a compatible version of chrome to `XDG_DATA_HOME` (or equivalent on Windows/Mac). This behaviour can be optionally turned off, and you can use the system version of chrome (assuming you have chrome installed) by disabling the default feature in your `Cargo.toml`:

```toml
[dependencies.headless_chrome]
default-features = false
```

## Missing features

- Frame / iframe support
- `window.alert` handlers
- Frankly, most of what's possible using the [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/tot)

## Contributing

Pull requests and issues are most welcome, even if they're just experience reports. If you find anything frustrating or confusing, let me know!
