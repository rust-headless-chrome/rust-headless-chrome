//! Create a headless browser, navigate to wikipedia, wait for the page
//! to render completely, take a screenshot in JPEG-format using 75% quality
use headless_chrome::{cdtp::page::ScreenshotFormat, Browser, LaunchOptions};
use std::fs;

fn main() -> Result<(), failure::Error> {
    let opts = LaunchOptions::default().expect("Could not find chrome");
    let browser = Browser::new(opts)?;
    let jpeg_data = browser
        .wait_for_initial_tab()?
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;
    println!("Screenshot successfully created.");
    Ok(())
}
