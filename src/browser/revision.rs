use failure::Error;
use log::*;
use reqwest;
use std::path::Path;
use xdg;

const APP_NAME: &str = "headless-chrome";

const REV_URL: &str =
    "https://storage.googleapis.com/chromium-browser-snapshots/Linux_x64/LAST_CHANGE";
#[cfg(target_os = "macos")]
const REV_URL: &str = "https://storage.googleapis.com/chromium-browser-snapshots/Mac/LAST_CHANGE";
#[cfg(windows)]
const REV_URL: &str =
    "https://storage.googleapis.com/chromium-browser-snapshots/Win_x64/LAST_CHANGE";

pub fn get_latest_rev() -> Result<String, Error> {
    info!("Getting latest chrome revision");
    let rev = reqwest::get(REV_URL)?.text()?;
    info!("Latest revision is: {}", rev);
    Ok(rev)
}

pub fn get_xdg_dir() -> Result<impl AsRef<Path>, Error> {
    let xdg_dir = xdg::BaseDirectories::with_prefix(APP_NAME)?;
    Ok(xdg_dir.get_data_home().as_path())
}
