use std::fs;

use failure::Fallible;

use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptions};

fn main() -> Fallible<()> {
    // Create a headless browser, navigate to wikipedia.org, wait for the page
    // to render completely, take a screenshot of the entire page
    // in JPEG-format using 75% quality.
    let options = LaunchOptions::default_builder()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    let jpeg_data = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), None, true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;

    // Browse to the WebKit-Page and take a screenshot of the infobox.
    let png_data = tab
        .navigate_to("https://en.wikipedia.org/wiki/WebKit")?
        .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    fs::write("screenshot.png", &png_data)?;

    println!("Screenshots successfully created.");
    Ok(())
}
