use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufWriter},
    path::{Path, PathBuf},
    str::FromStr,
};

use directories::ProjectDirs;
use failure::{format_err, Fallible};
use log::*;
use ureq;
use walkdir::WalkDir;
use zip;

pub const CUR_REV: &str = "634997";

const APP_NAME: &str = "headless-chrome";
const DEFAULT_HOST: &str = "https://storage.googleapis.com";

#[cfg(target_os = "linux")]
const PLATFORM: &str = "linux";
#[cfg(target_os = "macos")]
const PLATFORM: &str = "mac";
#[cfg(windows)]
const PLATFORM: &str = "win";

#[derive(Clone)]
pub struct FetcherOptions {
    /// The desired chrome revision.
    ///
    /// defaults to CUR_REV
    revision: String,

    /// The prefered installation directory. If not None we will look here first
    /// for existing installs.
    ///
    /// defaults to None
    install_dir: Option<PathBuf>,

    /// Allow headless_chrome to download and install the desired revision if not found.
    ///
    /// defaults to true
    allow_download: bool,

    /// Allow looking in the standard installation directories for existing installs.
    ///
    /// defaults to true
    allow_standard_dirs: bool,
}

impl Default for FetcherOptions {
    fn default() -> Self {
        Self {
            revision: CUR_REV.into(),
            install_dir: None,
            allow_download: true,
            allow_standard_dirs: true,
        }
    }
}

impl FetcherOptions {
    pub fn with_revision<S: Into<String>>(mut self, revision: S) -> Self {
        self.revision = revision.into();
        self
    }

    pub fn with_install_dir<P: Into<PathBuf>>(mut self, install_dir: Option<P>) -> Self {
        match install_dir {
            Some(dir) => self.install_dir = Some(dir.into()),
            None => self.install_dir = None,
        }
        self
    }

    pub fn with_allow_download(mut self, allow_download: bool) -> Self {
        self.allow_download = allow_download;
        self
    }

    pub fn with_allow_standard_dirs(mut self, allow_standard_dirs: bool) -> Self {
        self.allow_standard_dirs = allow_standard_dirs;
        self
    }
}

#[derive(Default)]
pub struct Fetcher {
    options: FetcherOptions,
}

impl Fetcher {
    pub fn new(options: FetcherOptions) -> Fallible<Self> {
        Ok(Self { options })
    }

    // look for good existing installation, if none exists then download and install
    pub fn fetch(&self) -> Fallible<PathBuf> {
        if let Ok(chrome_path) = self.chrome_path() {
            // we found it!
            return Ok(chrome_path);
        }

        if self.options.allow_download {
            let zip_path = self.download()?;

            self.unzip(zip_path)?;

            // look again
            return self.chrome_path();
        }

        // couldn't find and not allowed to download
        Err(format_err!("Could not fetch"))
    }

    // Look for an installation directory matching self.options.revision
    fn base_path(&self) -> Fallible<PathBuf> {
        // we want to look in install_dir first, then data dir
        let mut search_dirs: Vec<&Path> = Vec::new();
        let project_dirs = get_project_dirs()?;
        if let Some(install_dir) = &self.options.install_dir {
            search_dirs.push(install_dir.as_path());
        }
        if self.options.allow_standard_dirs {
            search_dirs.push(project_dirs.data_dir());
        }

        for root_dir in search_dirs {
            for entry in WalkDir::new(root_dir).into_iter().filter_map(Result::ok) {
                // filename is formatted as `{PLATFORM}-{REVISION}`
                let filename_parts = entry
                    .file_name()
                    .to_str()
                    .ok_or_else(|| format_err!("Failed conversion to UTF-8"))?
                    .split('-')
                    .collect::<Vec<_>>();

                if filename_parts.len() == 2
                    && filename_parts[0] == PLATFORM
                    && filename_parts[1] == self.options.revision
                {
                    return Ok(entry.path().into());
                }
            }
        }

        Err(format_err!("Could not find an existing revision"))
    }

    // find full path to chrome executable from base_path
    fn chrome_path(&self) -> Fallible<PathBuf> {
        let mut path = self.base_path()?;
        path.push(archive_name(&self.options.revision)?);

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

    // download a .zip of the revision we want
    fn download(&self) -> Fallible<PathBuf> {
        let url = dl_url(&self.options.revision)?;
        info!("Chrome download url: {}", url);
        let total = get_size(&url)?;
        info!("Total size of download: {} MiB", total);

        let mut path: PathBuf = if let Some(mut dir) = self.options.install_dir.clone() {
            // we have a preferred install location
            dir.push(format!("{}-{}", PLATFORM, self.options.revision));
            dir
        } else if self.options.allow_standard_dirs {
            let mut dir = get_project_dirs()?.data_dir().to_path_buf();
            dir.push(format!("{}-{}", PLATFORM, self.options.revision));
            dir
        } else {
            // No preferred install dir and not allowed to use standard dirs.
            // Not likely for someone to try and do this on purpose.
            return Err(format_err!("No allowed installation directory"));
        };
        path = path.with_extension("zip");
        // we need to create this directory in case it doesn't exist yet
        fs::create_dir_all(
            path.parent()
                .ok_or_else(|| format_err!("Path {:?} does not have a parent directory", path))?,
        )
        .map_err(|_err| format_err!("Could not create directory at {:?}", path.parent()))?;

        println!("{:?}", path);

        info!("Creating file for download: {}", &path.display());
        let mut file = OpenOptions::new().create(true).write(true).open(&path)?;

        let resp = ureq::get(&url).call();
        io::copy(&mut resp.into_reader(), &mut file)?;

        Ok(path)
    }

    // unzip the downloaded file and do all the needed file manipulation
    fn unzip<P: AsRef<Path>>(&self, zip_path: P) -> Fallible<PathBuf> {
        let mut archive = zip::ZipArchive::new(File::open(zip_path.as_ref())?)?;

        let mut extract_path: PathBuf = zip_path
            .as_ref()
            .parent()
            .ok_or_else(|| format_err!("zip_path does not have a parent directory"))?
            .to_path_buf();

        let folder_name = zip_path
            .as_ref()
            .file_stem()
            .ok_or_else(|| format_err!("zip_path does not have a file stem"))?;

        extract_path.push(folder_name);

        fs::create_dir_all(&extract_path)?;

        info!(
            "Extracting (this can take a while): {}",
            extract_path.display()
        );

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
        if fs::remove_file(&zip_path).is_err() {
            info!("Failed to delete zip");
            return Ok(extract_path);
        }

        Ok(extract_path)
    }
}

fn get_size<U: AsRef<str>>(url: U) -> Fallible<u64> {
    let resp = ureq::get(url.as_ref()).call();
    match resp.header("Content-Length") {
        Some(len) => Ok(u64::from_str(len)? / 2_u64.pow(20)),
        None => Err(format_err!("response doesn't include the content length")),
    }
}

fn get_project_dirs() -> Fallible<ProjectDirs> {
    info!("Getting project dir");
    match ProjectDirs::from("", "", APP_NAME) {
        Some(dirs) => Ok(dirs),
        None => Err(format_err!("Failed to retrieve project dirs")),
    }
}

fn dl_url<R>(revision: R) -> Fallible<String>
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

fn archive_name<R: AsRef<str>>(revision: R) -> Fallible<&'static str> {
    #[cfg(target_os = "linux")]
    {
        drop(revision);

        Ok("chrome-linux")
    }

    #[cfg(target_os = "macos")]
    {
        drop(revision);

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
