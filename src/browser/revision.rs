use directories::ProjectDirs;
use failure::Error;
use log::*;
use reqwest;

use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
};

const APP_NAME: &str = "headless-chrome";
const DEFAULT_HOST: &str = "https://storage.googleapis.com";

const REV_URL: &str =
    "https://storage.googleapis.com/chromium-browser-snapshots/Linux_x64/LAST_CHANGE";
#[cfg(target_os = "macos")]
const REV_URL: &str = "https://storage.googleapis.com/chromium-browser-snapshots/Mac/LAST_CHANGE";
#[cfg(windows)]
const REV_URL: &str =
    "https://storage.googleapis.com/chromium-browser-snapshots/Win_x64/LAST_CHANGE";

pub fn download<U, P>(url: U, path: P) -> Result<(), Error>
where
    U: AsRef<str>,
    P: AsRef<Path>,
{
    info!("Downloading file from: {}", url.as_ref());
    let mut resp = reqwest::get(url.as_ref())?;

    info!("Creating dir: {}", path.as_ref().display());
    let mut dest = File::create(path.as_ref())?;

    info!("Copying data into location");
    io::copy(&mut resp, &mut dest)?;
    Ok(())
}

pub fn get_latest_rev() -> Result<String, Error> {
    info!("Getting latest chrome revision");
    let rev = reqwest::get(REV_URL)?.text()?;
    info!("Latest revision is: {}", rev);
    Ok(rev)
}

pub fn get_project_dirs() -> Result<ProjectDirs, String> {
    info!("Getting project dir");
    match ProjectDirs::from("", "", APP_NAME) {
        Some(dirs) => Ok(dirs),
        None => Err("Failed to retrieve project dirs".to_string()),
    }
}

pub const fn dl_url<R>(revision: R) -> String
where
    R: AsRef<str>,
{
    return format!(
        "{}/chromium-browser-snapshots/Linux_x64/{}/{}.zip",
        DEFAULT_HOST,
        revision.as_ref(),
        archive_name(revision)
    );

    #[cfg(target_os = "macos")]
    return format!(
        "{}/chromium-browser-snapshots/Mac/{}/{}.zip",
        DEFAULT_HOST,
        revision.as_ref(),
        archive_name(revision)
    );

    #[cfg(windows)]
    return format!(
        "{}/chromium-browser-snapshots/Win_x64/{}/{}.zip",
        DEFAULT_HOST,
        revision.as_ref(),
        archive_name(revision)
    );
}

pub const fn archive_name<S: AsRef<str>>(revision: S) -> &'static str {
    #[cfg(target_os = "macos")]
    return "chrome-mac";

    #[cfg(windows)]
    // Windows archive name changed at r591479.
    return if revision.as_ref().parse::<u32>()? > 591479 {
        "chrome-win"
    } else {
        "chrome-win32"
    };

    "chrome-linux"
}
