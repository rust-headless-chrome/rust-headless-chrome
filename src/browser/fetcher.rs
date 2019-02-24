use directories::ProjectDirs;
use failure::{format_err, Error};
use indicatif::ProgressBar;
use log::*;
use reqwest::{self, header::CONTENT_LENGTH};

use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

pub const CUR_REV: &str = "634997";

const APP_NAME: &str = "headless-chrome";
const DEFAULT_HOST: &str = "https://storage.googleapis.com";

const REV_URL: &str =
    "https://storage.googleapis.com/chromium-browser-snapshots/Linux_x64/LAST_CHANGE";
#[cfg(target_os = "macos")]
const REV_URL: &str = "https://storage.googleapis.com/chromium-browser-snapshots/Mac/LAST_CHANGE";
#[cfg(windows)]
const REV_URL: &str =
    "https://storage.googleapis.com/chromium-browser-snapshots/Win_x64/LAST_CHANGE";

const PLATFORM: &str = "linux";
#[cfg(target_os = "macos")]
const PLATFORM: &str = "mac";
#[cfg(windows)]
const PLATFORM: &str = "win";

struct DownloadProgress<W, F> {
    inner: W,
    bytes_read: usize,
    total: u64,
    progress: F,
}

impl<W, F> DownloadProgress<W, F>
where
    W: Write,
    F: FnMut(usize),
{
    pub fn new(inner: W, total: u64, progress: F) -> Self {
        Self {
            total,
            inner,
            bytes_read: 0,
            progress,
        }
    }
}

impl<W: Write, F: FnMut(usize)> Write for DownloadProgress<W, F> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf).map(|n| {
            self.bytes_read += n;
            (self.progress)(self.bytes_read);
            n
        })
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

struct Revision {
    path: PathBuf,
    rev: String,
}

pub struct Fetcher<'a> {
    rev: &'a str,
    dirs: ProjectDirs,
}

impl<'a> Fetcher<'a> {
    pub fn new(rev: &'a str) -> Result<Self, Error> {
        let dirs = get_project_dirs()?;
        Ok(Self { rev, dirs })
    }

    fn local_revisions(&self) -> Result<Vec<Revision>, Error> {
        let mut revisions = Vec::new();
        for entry in fs::read_dir(self.dirs.data_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let filename = path
                    .file_name()
                    .ok_or_else(|| format_err!("Failed to turn into OsStr"))?
                    .to_str()
                    .ok_or_else(|| format_err!("Failed conversion to UTF8"))?
                    .split('-')
                    .collect::<Vec<_>>();
                if filename.len() == 2 && filename[0] == PLATFORM {
                    let rev = filename[1].to_string();
                    revisions.push(Revision { path, rev });
                }
            }
        }
        Ok(revisions)
    }

    fn rev_path(&self) -> PathBuf {
        let mut path = self.dirs.data_dir().to_path_buf();
        path.push(format!("{}-{}", PLATFORM, self.rev));
        path
    }

    pub fn run(&self) -> Result<PathBuf, Error> {
        let revisions = self.local_revisions()?;
        if let Some(revision) = revisions.into_iter().find(|r| r.rev == self.rev) {
            info!("No need to download, we have the correct revision");
            return Ok(revision.path);
        }
        let url = dl_url(self.rev);
        let total = get_size(&url)?;
        let path = self.rev_path();
        let file = File::create(&path)?;
        let pb = ProgressBar::new(total);
        let mut dest = DownloadProgress::new(file, total, |n| pb.set_position(n as u64));

        info!("Downloading file from: {}", &url);
        let mut resp = reqwest::get(&url)?;

        info!("Copying data into location");
        io::copy(&mut resp, &mut dest)?;

        Ok(path)
    }
}

fn get_size<U: AsRef<str>>(url: U) -> Result<u64, Error> {
    let client = reqwest::Client::new();
    let response = client.head(url.as_ref()).send()?;
    match response.headers().get(CONTENT_LENGTH) {
        Some(len) => {
            let length = u64::from_str(len.to_str()?)?;
            Ok(length)
        }
        None => Err(format_err!("response doesn't include the content length")),
    }
}

fn get_latest_rev() -> Result<String, Error> {
    info!("Getting latest chrome revision");
    let rev = reqwest::get(REV_URL)?.text()?;
    info!("Latest revision is: {}", rev);
    Ok(rev)
}

fn get_project_dirs() -> Result<ProjectDirs, Error> {
    info!("Getting project dir");
    match ProjectDirs::from("", "", APP_NAME) {
        Some(dirs) => Ok(dirs),
        None => Err(format_err!("Failed to retrieve project dirs")),
    }
}

fn dl_url<R>(revision: R) -> String
where
    R: AsRef<str>,
{
    return format!(
        "{}/chromium-browser-snapshots/Linux_x64/{}/{}.zip",
        DEFAULT_HOST,
        revision.as_ref(),
        archive_name(revision.as_ref())
    );

    #[cfg(target_os = "macos")]
    return format!(
        "{}/chromium-browser-snapshots/Mac/{}/{}.zip",
        DEFAULT_HOST,
        revision.as_ref(),
        archive_name(revision.as_ref())
    );

    #[cfg(windows)]
    return format!(
        "{}/chromium-browser-snapshots/Win_x64/{}/{}.zip",
        DEFAULT_HOST,
        revision.as_ref(),
        archive_name(revision.as_ref())
    );
}

fn archive_name<R: AsRef<str>>(revision: R) -> &'static str {
    #[cfg(target_os = "macos")]
    {
        return "chrome-mac";
    }

    #[cfg(windows)]
    {
        // Windows archive name changed at r591479.
        return if revision.as_ref().parse::<u32>()? > 591479 {
            "chrome-win"
        } else {
            "chrome-win32"
        };
    }

    "chrome-linux"
}
