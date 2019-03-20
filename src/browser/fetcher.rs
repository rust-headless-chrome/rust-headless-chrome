use directories::ProjectDirs;
use failure::{format_err, Error};
use log::*;
use reqwest::{self, header::CONTENT_LENGTH};
use zip;

use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufWriter},
    path::{Path, PathBuf},
    str::FromStr,
};

pub const CUR_REV: &str = "634997";

const APP_NAME: &str = "headless-chrome";
const DEFAULT_HOST: &str = "https://storage.googleapis.com";

#[cfg(target_os = "linux")]
const PLATFORM: &str = "linux";
#[cfg(target_os = "macos")]
const PLATFORM: &str = "mac";
#[cfg(windows)]
const PLATFORM: &str = "win";

pub struct Fetcher<'a> {
    rev: &'a str,
    dirs: ProjectDirs,
}

impl<'a> Fetcher<'a> {
    pub fn new(rev: &'a str) -> Result<Self, Error> {
        let dirs = get_project_dirs()?;
        info!(
            "Creating XDG_DATA_DIR if it doesn't exist: {}",
            dirs.data_dir().display()
        );
        fs::create_dir_all(dirs.data_dir())?;
        Ok(Self { rev, dirs })
    }

    fn local_revisions(&self) -> Result<Vec<String>, Error> {
        trace!(
            "Enumerating contents of XDG_DATA_DIR: {}",
            self.dirs.data_dir().display()
        );
        let mut revisions = Vec::new();
        for entry in fs::read_dir(self.dirs.data_dir())? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let filename = path
                    .file_name()
                    .ok_or_else(|| format_err!("Failed to turn into OsStr"))?
                    .to_str()
                    .ok_or_else(|| format_err!("Failed conversion to UTF8"))?
                    .split('-')
                    .collect::<Vec<_>>();
                if filename.len() == 2 && filename[0] == PLATFORM {
                    let rev = filename[1].to_string();
                    revisions.push(rev)
                }
            }
        }
        Ok(revisions)
    }

    fn base_path(&self, rev: &str) -> PathBuf {
        let mut path = self.dirs.data_dir().to_path_buf();
        path.push(format!("{}-{}", PLATFORM, rev));
        path
    }

    fn chrome_path(&self, rev: &str) -> Result<PathBuf, Error> {
        let mut path = self.base_path(rev);
        path.push(archive_name(rev)?);

        #[cfg(target_os = "linux")]
        {
            path.push("chrome");
        }
        #[cfg(target_os = "macos")]
        {
            path.push("Chromium.app");
            path.push("Contents");
            path.push("MacOS");
            path.push("Chromium");
        }
        #[cfg(windows)]
        {
            path.push("chrome.exe");
        }

        Ok(path)
    }

    pub fn run(&self) -> Result<PathBuf, Error> {
        let revisions = self.local_revisions()?;
        if let Some(revision) = revisions.into_iter().find(|r| r == self.rev) {
            info!("No need to download, we have the correct revision");
            return Ok(self.chrome_path(&revision)?);
        }

        let url = dl_url(self.rev)?;
        info!("Chrome download url: {}", url);
        let total = get_size(&url)?;
        info!("Total size of download: {}", total);
        let path = self.base_path(self.rev).with_extension("zip");

        info!("Creating file for download: {}", &path.display());
        let mut file = OpenOptions::new().create(true).write(true).open(&path)?;

        let mut resp = reqwest::get(&url)?;
        io::copy(&mut resp, &mut file)?;

        self.unzip(&path)?;

        Ok(self.chrome_path(self.rev)?)
    }

    pub fn unzip<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        let mut archive = zip::ZipArchive::new(File::open(path.as_ref())?)?;
        let extract_path = self.base_path(self.rev);
        fs::create_dir_all(&extract_path)?;

        info!("Extracting: {}", extract_path.display());

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let mut out_path = extract_path.clone();
            out_path.push(file.sanitized_name().as_path());

            let comment = file.comment();
            if !comment.is_empty() {
                trace!("File {} comment: {}", i, comment);
            }

            if (&*file.name()).ends_with('/') {
                trace!(
                    "File {} extracted to \"{}\"",
                    i,
                    out_path.as_path().display()
                );
                fs::create_dir_all(&out_path)?;
            } else {
                trace!(
                    "File {} extracted to \"{}\" ({} bytes)",
                    i,
                    out_path.as_path().display(),
                    file.size()
                );
                if let Some(p) = out_path.parent() {
                    if !p.exists() {
                        fs::create_dir_all(&p).unwrap();
                    }
                }
                let mut out_file = BufWriter::new(File::create(&out_path)?);
                io::copy(&mut file, &mut out_file)?;
            }
            // Get and Set permissions
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                if let Some(mode) = file.unix_mode() {
                    fs::set_permissions(&out_path, fs::Permissions::from_mode(mode)).unwrap();
                }
            }
        }

        info!("Cleaning up");
        if fs::remove_file(&path).is_err() {
            info!("Failed to delete zip");
            return Ok(());
        }

        Ok(())
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

fn get_project_dirs() -> Result<ProjectDirs, Error> {
    info!("Getting project dir");
    match ProjectDirs::from("", "", APP_NAME) {
        Some(dirs) => Ok(dirs),
        None => Err(format_err!("Failed to retrieve project dirs")),
    }
}

fn dl_url<R>(revision: R) -> Result<String, Error>
where
    R: AsRef<str>,
{
    #[cfg(target_os = "linux")]
    {
        Ok(format!(
            "{}/chromium-browser-snapshots/Linux_x64/{}/{}.zip",
            DEFAULT_HOST,
            revision.as_ref(),
            archive_name(revision.as_ref())?
        ))
    }

    #[cfg(target_os = "macos")]
    {
        Ok(format!(
            "{}/chromium-browser-snapshots/Mac/{}/{}.zip",
            DEFAULT_HOST,
            revision.as_ref(),
            archive_name(revision.as_ref())?
        ))
    }

    #[cfg(windows)]
    {
        Ok(format!(
            "{}/chromium-browser-snapshots/Win_x64/{}/{}.zip",
            DEFAULT_HOST,
            revision.as_ref(),
            archive_name(revision.as_ref())?
        ))
    }
}

fn archive_name<R: AsRef<str>>(_revision: R) -> Result<&'static str, Error> {
    #[cfg(target_os = "linux")]
    {
        Ok("chrome-linux")
    }

    #[cfg(target_os = "macos")]
    {
        Ok("chrome-mac")
    }

    #[cfg(windows)]
    {
        // Windows archive name changed at r591479.
        if revision.as_ref().parse::<u32>()? > 591_479 {
            Ok("chrome-win")
        } else {
            Ok("chrome-win32")
        }
    }
}
