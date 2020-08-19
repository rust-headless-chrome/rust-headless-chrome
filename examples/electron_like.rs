use failure::Fallible;

use headless_chrome::{Browser, LaunchOptionsBuilder};

// This could possibly be used to integrate Rust as a way to create web-based desktop applications
fn main() -> Fallible<()> {
    // Launch chrome and point it to GitHub in "app" mode, which causes it to
    // run without the browser tabs and headers, similar to electron.
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .default_args(false)
        .app_url(Some(
            "https://github.com/atroche/rust-headless-chrome/issues".into(),
        ))
        .build()
        .expect("Couldn't find appropriate Chrome binary.");
    let _browser_handle = Browser::new(options)?;

    // Wait 15 seconds so you can browse manually and test it
    std::thread::sleep(std::time::Duration::from_secs(15));

    Ok(())
}
