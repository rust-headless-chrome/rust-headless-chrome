# Headless Chrome

[![Build Status](https://github.com/atroche/rust-headless-chrome/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/atroche/rust-headless-chrome/actions/workflows/ci.yml)
[![Crate](https://img.shields.io/crates/v/headless_chrome.svg)](https://crates.io/crates/headless_chrome)
[![API](https://docs.rs/headless_chrome/badge.svg)](https://docs.rs/headless_chrome)
[![Discord channel](https://img.shields.io/discord/557374784233799681.svg?logo=discord)](https://discord.gg/yyGEzcc)

A high-level API to control headless Chrome or Chromium over the DevTools Protocol. It is the
Rust equivalent of [Puppeteer](https://github.com/GoogleChrome/puppeteer), a Node library
maintained by the Chrome DevTools team.

It is not 100% feature compatible with Puppeteer, but there's enough here to satisfy most
browser testing / web crawling use cases, and there are several 'advanced' features such as:

- [network request interception](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.enable_request_interception)
- [JavaScript coverage monitoring](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.take_precise_js_coverage)
- Opening incognito windows
- [taking screenshots of elements or the entire page](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.capture_screenshot)
- [saving pages to PDF](https://docs.rs/headless_chrome/latest/headless_chrome/browser/tab/struct.Tab.html#method.print_to_pdf)
- ['headful' browsing](https://docs.rs/headless_chrome/latest/headless_chrome/struct.LaunchOptionsBuilder.html#method.headless)
- automatic downloading of 'known good' Chromium binaries for Linux / Mac / Windows
- [extension pre-loading](https://docs.rs/headless_chrome/latest/headless_chrome/struct.LaunchOptionsBuilder.html#method.extensions)

## Quick Start

```rust
use std::error::Error;

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

fn browse_wikipedia() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;

    let tab = browser.new_tab()?;

    /// Navigate to wikipedia
    tab.navigate_to("https://www.wikipedia.org")?;

    /// Wait for network/javascript/dom to make the search-box available
    /// and click it.
    tab.wait_for_element("input#searchInput")?.click()?;

    /// Type in a query and press `Enter`
    tab.type_str("WebKit")?.press_key("Enter")?;

    /// We should end up on the WebKit-page once navigated
    let elem = tab.wait_for_element("#firstHeading")?;
    assert!(tab.get_url().ends_with("WebKit"));

    /// Take a screenshot of the entire browser window
    let _jpeg_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true)?;

    /// Take a screenshot of just the WebKit-Infobox
    let _png_data = tab
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;

    // Run JavaScript in the page
    let remote_object = elem.call_js_fn(r#"
        function getIdTwice () {
            // `this` is always the element that you called `call_js_fn` on
            const id = this.id;
            return id + id;
        }
    "#, vec![], false)?;
    match remote_object.value {
        Some(returned_string) => {
            dbg!(&returned_string);
            assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
        }
        _ => unreachable!()
    };

    Ok(())
}
```

# Auto fetching chrome binary

```toml
[dependencies]
headless_chrome = {git = "https://github.com/rust-headless-chrome/rust-headless-chrome", features = ["fetch"]}
```



For fuller examples, take a look at [`tests/simple.rs`](tests/simple.rs) and [`examples`](examples/).

> Before running examples. Make sure add [failure](https://crates.io/crates/failure) crate in your cargo project dependency of `Cargo.toml`


## What can't it do?

The [Chrome DevTools Protocol](https://chromedevtools.github.io/devtools-protocol/tot/Browser) is huge. Currently, Puppeteer supports way more of it than we do. Some of the missing features include:

-  Dealing with frames
-  Handling file picker / chooser interactions
-  Tapping touchscreens
-  Emulating different network conditions (DevTools can alter latency, throughput, offline status, 'connection type')
-  Viewing timing information about network requests
-  Reading the SSL certificate
-  Replaying XHRs
-  HTTP Basic Auth
-  Inspecting `EventSource`s (aka server-sent events or SSEs)
-  WebSocket inspection

If you're interested in adding one of these features but would like some advice about how to start, please reach out by creating an issue or sending me an email at [`alistair@sunburnt.country`](mailto:alistair@sunburnt.country).

## Related crates

-  [fantoccini](https://github.com/jonhoo/fantoccini) uses WebDriver, so it works with browsers other than Chrome. It's also asynchronous and based on Tokio, unlike `headless_chrome`, which has a synchronous API and is just implemented using plain old threads. Fantoccini has also been around longer and is more battle-tested. It doesn't support Chrome DevTools-specific functionality like JS Coverage.

## Testing

For debug output, set these environment variables before running `cargo test`:

```RUST_BACKTRACE=1 RUST_LOG=headless_chrome=trace```

## Version numbers

Starting with v0.2.0, we're trying to follow SemVar strictly.

## Troubleshooting

If you get errors related to timeouts, you likely need to enable sandboxing either in the kernel or as a setuid sandbox. Puppeteer has some information about how to do that [here](https://github.com/GoogleChrome/puppeteer/blob/master/docs/troubleshooting.md)

By default, `headless_chrome` will download a compatible version of chrome to `XDG_DATA_HOME` (or equivalent on Windows/Mac). This behaviour can be optionally turned off, and you can use the system version of chrome (assuming you have chrome installed) by disabling the default feature in your `Cargo.toml`:

```toml
[dependencies.headless_chrome]
default-features = false
```

## Contributing

Pull requests and issues are most welcome, even if they're just experience reports. If you find anything frustrating or confusing, let me know!
