//! Create a headless browser, navigate to wikipedia, wait for the page
//! to render completely, take a screenshot in JPEG-format using 75% quality
use headless_chrome::{cdtp::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
use std::fs;

fn main() -> Result<(), failure::Error> {
    let options = LaunchOptionsBuilder::default()
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let browser = Browser::new(options).expect("Failed to launch and connect to Chrome");
    let jpeg_data = browser
        .wait_for_initial_tab()?
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .capture_screenshot(ScreenshotFormat::JPEG(Some(75)), true)?;
    fs::write("screenshot.jpg", &jpeg_data)?;
    println!("Screenshot successfully created.");
    Ok(())
}
