use std::{
    borrow::BorrowMut,
    ffi::OsStr,
    io::{BufRead, BufReader, prelude::*},
    net,
    process::{Child, Command, Stdio},
    time::Duration,
};

#[cfg(test)]
use std::cell::RefCell;

use derive_builder::Builder;

use anyhow::{Result, anyhow};
use log::*;
use rand::rng;
use rand::seq::SliceRandom;
use regex::Regex;
use thiserror::Error;
use url::Url;
#[cfg(windows)]
use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

#[cfg(not(feature = "fetch"))]
use crate::browser::default_executable;
use crate::util;

#[cfg(feature = "fetch")]
use super::fetcher::{Fetcher, FetcherOptions};
use std::collections::HashMap;

#[cfg(test)]
struct ForTesting;
#[cfg(test)]
impl ForTesting {
    thread_local! {
        static USER_DATA_DIR: RefCell<Option<String>> = const { RefCell::new(None) };
    }
}

pub struct Process {
    child_process: TemporaryProcess,
    pub debug_ws_url: Url,
}

#[derive(Debug, Error)]
enum ChromeLaunchError {
    #[error("Chrome launched, but didn't give us a WebSocket URL before we timed out")]
    PortOpenTimeout,
    #[error("There are no available ports between 8000 and 9000 for debugging")]
    NoAvailablePorts,
    #[error("The chosen debugging port is already in use")]
    DebugPortInUse,
    #[error("You need to set the sandbox(false) option when running as root")]
    RunningAsRootWithoutNoSandbox,
}

#[cfg(windows)]
pub(crate) fn get_chrome_path_from_registry() -> Option<std::path::PathBuf> {
    RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\chrome.exe")
        .and_then(|key| key.get_value::<String, _>(""))
        .map(std::path::PathBuf::from)
        .ok()
}

struct TemporaryProcess(Child, Option<tempfile::TempDir>);

impl Drop for TemporaryProcess {
    fn drop(&mut self) {
        info!("Killing Chrome. PID: {}", self.0.id());
        self.0.kill().and_then(|()| self.0.wait()).ok();
        if let Some(dir) = self.1.take() {
            if let Err(e) = dir.close() {
                warn!("Failed to close temporary directory: {}", e);
            }
        };
    }
}

/// Represents the way in which Chrome is run. By default it will search for a Chrome
/// binary on the system, use an available port for debugging, and start in headless mode.
#[derive(Clone, Debug, Builder)]
pub struct LaunchOptions<'a> {
    /// Determines whether to run headless version of the browser. Defaults to true.
    #[builder(default = "true")]
    pub headless: bool,

    /// Determines whether to run the browser with a sandbox.
    #[builder(default = "true")]
    pub sandbox: bool,

    /// Automatically open devtools for tabs. Forces headless to be false
    #[builder(default = "false")]
    pub devtools: bool,

    /// Determines whether to enable GPU or not. Default to false.
    #[builder(default = "false")]
    pub enable_gpu: bool,

    /// Determines whether to run the browser with logging enabled
    /// (this can cause unwanted new shell window in Windows 10 and above).
    /// Check <https://github.com/rust-headless-chrome/rust-headless-chrome/issues/371>
    #[builder(default = "false")]
    pub enable_logging: bool,

    /// Launch the browser with a specific window width and height.
    #[builder(default = "None")]
    pub window_size: Option<(u32, u32)>,

    /// Launch the browser with a specific debugging port.
    #[builder(default = "None")]
    pub port: Option<u16>,
    /// Determines whether SSL certificates should be verified.
    /// This is unsafe and can lead to MiTM attacks. Make sure you understand the risks
    /// See <https://www.owasp.org/index.php/Man-in-the-middle_attack>
    #[builder(default = "true")]
    pub ignore_certificate_errors: bool,

    /// Path for Chrome or Chromium.
    ///
    /// If unspecified, the create will try to automatically detect a suitable binary.
    #[builder(default = "None")]
    pub path: Option<std::path::PathBuf>,

    /// User Data (Profile) to use.
    ///
    /// If unspecified, a new temp directory is created and used on every launch.
    #[builder(default = "None")]
    pub user_data_dir: Option<std::path::PathBuf>,

    /// A list of Chrome extensions to load.
    ///
    /// An extension should be a path to a folder containing the extension code.
    /// CRX files cannot be used directly and must be first extracted.
    ///
    /// Note that Chrome does not support loading extensions in headless-mode.
    /// See <https://bugs.chromium.org/p/chromium/issues/detail?id=706008#c5>
    #[builder(default)]
    pub extensions: Vec<&'a OsStr>,

    /// Additional arguments to pass to the browser instance. The list of Chromium
    /// flags can be found: <http://peter.sh/experiments/chromium-command-line-switches/>.
    #[builder(default)]
    pub args: Vec<&'a OsStr>,

    /// Ignore a default given flag
    #[builder(default)]
    pub ignore_default_args: Vec<&'a OsStr>,

    /// Disable default arguments
    #[builder(default)]
    pub disable_default_args: bool,

    /// The options to use for fetching a version of chrome when `path` is None.
    ///
    /// By default, we'll use a revision guaranteed to work with our API and will
    /// download and install that revision of chrome the first time a Process is created.
    #[cfg_attr(feature = "fetch", builder(default))]
    #[cfg(feature = "fetch")]
    pub fetcher_options: FetcherOptions,

    /// How long to keep the WebSocket to the browser for after not receiving any events from it
    /// Defaults to 30 seconds
    #[builder(default = "Duration::from_secs(30)")]
    pub idle_browser_timeout: Duration,

    /// Environment variables to set for the Chromium process.
    /// Passes value through to std::process::Command::envs.
    #[builder(default = "None")]
    pub process_envs: Option<HashMap<String, String>>,

    /// Setup the proxy server for headless chrome instance
    #[builder(default = "None")]
    pub proxy_server: Option<&'a str>,
}

impl Default for LaunchOptions<'_> {
    fn default() -> Self {
        LaunchOptions {
            headless: true,
            devtools: false,
            sandbox: true,
            enable_gpu: false,
            enable_logging: false,
            idle_browser_timeout: Duration::from_secs(30),
            window_size: None,
            path: None,
            user_data_dir: None,
            port: None,
            ignore_certificate_errors: true,
            extensions: Vec::new(),
            process_envs: None,
            #[cfg(feature = "fetch")]
            fetcher_options: Default::default(),
            args: Vec::new(),
            ignore_default_args: Vec::new(),
            disable_default_args: false,
            proxy_server: None,
        }
    }
}

impl<'a> LaunchOptions<'a> {
    pub fn default_builder() -> LaunchOptionsBuilder<'a> {
        LaunchOptionsBuilder::default()
    }
}

/// These are passed to the Chrome binary by default.
/// Via <https://github.com/GoogleChrome/puppeteer/blob/master/lib/Launcher.js#L38>
pub static DEFAULT_ARGS: [&str; 23] = [
    "--disable-background-networking",
    "--enable-features=NetworkService,NetworkServiceInProcess",
    "--disable-background-timer-throttling",
    "--disable-backgrounding-occluded-windows",
    "--disable-breakpad",
    "--disable-client-side-phishing-detection",
    "--disable-component-extensions-with-background-pages",
    "--disable-default-apps",
    "--disable-dev-shm-usage",
    "--disable-extensions",
    // BlinkGenPropertyTrees disabled due to crbug.com/937609
    "--disable-features=TranslateUI,BlinkGenPropertyTrees",
    "--disable-hang-monitor",
    "--disable-ipc-flooding-protection",
    "--disable-popup-blocking",
    "--disable-prompt-on-repost",
    "--disable-renderer-backgrounding",
    "--disable-sync",
    "--force-color-profile=srgb",
    "--metrics-recording-only",
    "--no-first-run",
    "--enable-automation",
    "--password-store=basic",
    "--use-mock-keychain",
];

impl Process {
    pub fn new(mut launch_options: LaunchOptions) -> Result<Self> {
        if launch_options.path.is_none() {
            #[cfg(feature = "fetch")]
            {
                let fetch = Fetcher::new(launch_options.fetcher_options.clone());
                launch_options.path = Some(fetch.fetch()?);
            }
            #[cfg(not(feature = "fetch"))]
            {
                launch_options.path = Some(default_executable().map_err(|e| anyhow!("{}", e))?);
            }
        }

        let mut process = Self::start_process(&launch_options)?;

        info!("Started Chrome. PID: {}", process.0.id());

        let url;
        let mut attempts = 0;
        loop {
            if attempts > 10 {
                return Err(ChromeLaunchError::NoAvailablePorts {}.into());
            }

            match Self::ws_url_from_output(process.0.borrow_mut()) {
                Ok(debug_ws_url) => {
                    url = debug_ws_url;
                    debug!("Found debugging WS URL: {:?}", url);
                    break;
                }
                Err(error) => {
                    trace!("Problem getting WebSocket URL from Chrome: {}", error);

                    if let Some(&ChromeLaunchError::RunningAsRootWithoutNoSandbox) =
                        error.downcast_ref::<ChromeLaunchError>()
                    {
                        return Err(error);
                    }

                    if launch_options.port.is_none() {
                        process = Self::start_process(&launch_options)?;
                    } else {
                        return Err(error);
                    }
                }
            }

            trace!(
                "Trying again to find available debugging port. Attempts: {}",
                attempts
            );
            attempts += 1;
        }

        let child = process.0.borrow_mut();
        child.stderr = None;

        Ok(Self {
            child_process: process,
            debug_ws_url: url,
        })
    }

    fn start_process(launch_options: &LaunchOptions) -> Result<TemporaryProcess> {
        let debug_port = if let Some(port) = launch_options.port {
            port
        } else {
            get_available_port().ok_or(ChromeLaunchError::NoAvailablePorts {})?
        };
        let port_option = format!("--remote-debugging-port={debug_port}");

        let window_size_option = if let Some((width, height)) = launch_options.window_size {
            format!("--window-size={width},{height}")
        } else {
            String::new()
        };

        let mut temp_user_data_dir = None;

        // User data directory
        let user_data_dir = if let Some(dir) = &launch_options.user_data_dir {
            dir.clone()
        } else {
            // picking random data dir so that each a new browser instance is launched
            // (see man google-chrome)
            let dir = ::tempfile::Builder::new()
                .prefix("rust-headless-chrome-profile")
                .tempdir()?;

            let buf = dir.path().to_path_buf();
            temp_user_data_dir = Some(dir);
            buf
        };
        let data_dir_option = format!("--user-data-dir={}", &user_data_dir.to_str().unwrap());

        #[cfg(test)]
        ForTesting::USER_DATA_DIR.with(|dir| {
            *dir.borrow_mut() = user_data_dir.to_str().map(std::borrow::ToOwned::to_owned);
        });

        trace!("Chrome will have profile: {}", data_dir_option);

        let mut args = vec![
            port_option.as_str(),
            "--verbose",
            "--log-level=0",
            "--no-first-run",
            data_dir_option.as_str(),
        ];

        if !launch_options.disable_default_args {
            let ignore_default_args: Vec<&str> = launch_options
                .ignore_default_args
                .iter()
                .map(|arg| arg.to_str().unwrap())
                .collect();

            let defaults: Vec<_> = DEFAULT_ARGS
                .iter()
                .filter(|arg| !ignore_default_args.contains(arg))
                .collect();

            args.extend(defaults);
        }

        if !launch_options.args.is_empty() {
            let extra_args: Vec<&str> = launch_options
                .args
                .iter()
                .map(|a| a.to_str().unwrap())
                .collect();
            args.extend(extra_args);
        }

        if !window_size_option.is_empty() {
            args.extend([window_size_option.as_str()]);
        }

        if launch_options.headless && !launch_options.devtools {
            args.extend(["--headless"]);
        } else if launch_options.devtools {
            args.extend(["--auto-open-devtools-for-tabs"]);
        }

        if launch_options.ignore_certificate_errors {
            args.extend(["--ignore-certificate-errors"]);
        }

        if launch_options.enable_logging {
            args.extend(["--enable-logging"]);
        }

        if !launch_options.enable_gpu {
            args.extend(["--disable-gpu"]);
        }

        let proxy_server_option = if let Some(proxy_server) = launch_options.proxy_server {
            format!("--proxy-server={proxy_server}")
        } else {
            String::new()
        };

        if !proxy_server_option.is_empty() {
            args.extend([proxy_server_option.as_str()]);
        }

        if !launch_options.sandbox {
            args.extend(["--no-sandbox", "--disable-setuid-sandbox"]);
        }

        let extension_args: Vec<String> = launch_options
            .extensions
            .iter()
            .map(|e| format!("--load-extension={}", e.to_str().unwrap()))
            .collect();

        args.extend(extension_args.iter().map(String::as_str));

        let path = launch_options
            .path
            .as_ref()
            .ok_or_else(|| anyhow!("Chrome path required"))?;

        info!("Launching Chrome binary at {:?}", &path);
        trace!("with CLI arguments: {:?}", args);

        let mut command = Command::new(path);

        if let Some(process_envs) = launch_options.process_envs.clone() {
            command.envs(process_envs);
        }

        // Suppress creation of a console window on windows
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        let process = TemporaryProcess(
            command.args(&args).stderr(Stdio::piped()).spawn()?,
            temp_user_data_dir,
        );
        Ok(process)
    }

    fn ws_url_from_reader<R>(reader: BufReader<R>) -> Result<Option<String>>
    where
        R: Read,
    {
        let port_taken_re = Regex::new(r"ERROR.*bind\(\)")?;
        let root_sandbox = "Running as root without --no-sandbox is not supported";

        let re = Regex::new(r"listening on (.*/devtools/browser/.*)$")?;

        let extract = |text: &str| -> Option<String> {
            let caps = re.captures(text);
            let cap = &caps?[1];
            Some(cap.into())
        };

        for line in reader.lines() {
            let chrome_output = line?;
            trace!("Chrome output: {}", chrome_output);

            if chrome_output.contains(root_sandbox) {
                return Err(ChromeLaunchError::RunningAsRootWithoutNoSandbox {}.into());
            }

            if port_taken_re.is_match(&chrome_output) {
                return Err(ChromeLaunchError::DebugPortInUse {}.into());
            }

            if let Some(answer) = extract(&chrome_output) {
                return Ok(Some(answer));
            }
        }

        Ok(None)
    }

    fn ws_url_from_output(child_process: &mut Child) -> Result<Url> {
        let chrome_output_result = util::Wait::with_timeout(Duration::from_secs(30)).until(|| {
            let my_stderr = BufReader::new(child_process.stderr.as_mut()?);
            match Self::ws_url_from_reader(my_stderr) {
                Ok(output_option) => output_option.map(Ok),
                Err(err) => Some(Err(err)),
            }
        });

        if let Ok(output_result) = chrome_output_result {
            Ok(Url::parse(&output_result?)?)
        } else {
            Err(ChromeLaunchError::PortOpenTimeout {}.into())
        }
    }

    pub fn get_id(&self) -> u32 {
        self.child_process.0.id()
    }
}

fn get_available_port() -> Option<u16> {
    let mut ports: Vec<u16> = (8000..9000).collect();
    ports.shuffle(&mut rng());
    ports.iter().find(|port| port_is_available(**port)).copied()
}

fn port_is_available(port: u16) -> bool {
    net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "fetch")]
    use std::fs;
    #[cfg(feature = "fetch")]
    use std::path::PathBuf;

    use std::sync::Once;
    use std::thread;

    use crate::browser::default_executable;

    use super::*;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env_logger::try_init().unwrap_or(());
        });
    }

    #[test]
    fn can_launch_chrome_and_get_ws_url() {
        setup();
        let chrome = super::Process::new(
            LaunchOptions::default_builder()
                .path(Some(default_executable().unwrap()))
                .build()
                .unwrap(),
        )
        .unwrap();
        info!("{:?}", chrome.debug_ws_url);
    }

    #[test]
    #[cfg(feature = "fetch")]
    fn can_install_chrome_to_dir_and_launch() {
        use crate::browser::fetcher::CUR_REV;
        #[cfg(target_os = "linux")]
        const PLATFORM: &str = "linux";
        #[cfg(target_os = "macos")]
        const PLATFORM: &str = "mac";
        #[cfg(windows)]
        const PLATFORM: &str = "win";

        let tests_temp_dir = [env!("CARGO_MANIFEST_DIR"), "tests", "temp"]
            .iter()
            .collect::<PathBuf>();

        setup();

        // clean up any artifacts from a previous run of this test.
        // if we do this after it fails on windows because chrome can stay running
        // for a bit.
        let mut installed_dir = tests_temp_dir.clone();
        installed_dir.push(format!("{PLATFORM}-{CUR_REV}"));

        if installed_dir.exists() {
            info!("Deleting pre-existing install at {:?}", &installed_dir);
            fs::remove_dir_all(&installed_dir).expect("Could not delete pre-existing install");
        }

        let chrome = super::Process::new(
            LaunchOptions::default_builder()
                .fetcher_options(FetcherOptions::default().with_install_dir(Some(&tests_temp_dir)))
                .build()
                .unwrap(),
        )
        .unwrap();
        info!("{:?}", chrome.debug_ws_url);
    }

    #[test]
    fn handle_errors_in_chrome_output() {
        setup();
        let lines = "[0228/194641.093619:ERROR:socket_posix.cc(144)] bind() returned an error, errno=0: Cannot assign requested address (99)";
        let reader = BufReader::new(lines.as_bytes());
        let ws_url_result = Process::ws_url_from_reader(reader);
        assert!(ws_url_result.is_err());
    }

    #[test]
    fn handle_errors_in_chrome_output_gvisor_netlink() {
        // see https://github.com/atroche/rust-headless-chrome/issues/261
        setup();
        let lines = "[0703/145506.975691:ERROR:address_tracker_linux.cc(214)] Could not bind NETLINK socket: Permission denied (13)";

        let reader = BufReader::new(lines.as_bytes());
        let ws_url_result = Process::ws_url_from_reader(reader);
        assert!(ws_url_result.is_ok());
    }

    #[cfg(target_os = "linux")]
    fn current_child_pids() -> Vec<i32> {
        use std::fs::File;
        use std::io::prelude::*;
        let current_pid = std::process::id();
        let mut current_process_children_file =
            File::open(format!("/proc/{current_pid}/task/{current_pid}/children")).unwrap();
        let mut child_pids = String::new();
        current_process_children_file
            .read_to_string(&mut child_pids)
            .unwrap();
        child_pids
            .split_whitespace()
            .map(|pid_str| pid_str.parse::<i32>().unwrap())
            .collect()
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn kills_process_on_drop() {
        setup();
        {
            let _chrome = &mut super::Process::new(
                LaunchOptions::default_builder()
                    .path(Some(default_executable().unwrap()))
                    .build()
                    .unwrap(),
            )
            .unwrap();
        }

        let child_pids = current_child_pids();
        assert!(child_pids.is_empty());
    }

    #[test]
    fn launch_multiple_non_headless_instances() {
        setup();
        let mut handles = Vec::new();

        for _ in 0..10 {
            let handle = thread::spawn(|| {
                // these sleeps are to make it more likely the chrome startups will overlap
                std::thread::sleep(std::time::Duration::from_millis(10));
                let chrome = super::Process::new(
                    LaunchOptions::default_builder()
                        .path(Some(default_executable().unwrap()))
                        .build()
                        .unwrap(),
                )
                .unwrap();
                std::thread::sleep(std::time::Duration::from_millis(100));
                chrome.debug_ws_url
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn no_instance_sharing() {
        setup();

        let mut handles = Vec::new();

        for _ in 0..10 {
            let chrome = super::Process::new(
                LaunchOptions::default_builder()
                    .path(Some(default_executable().unwrap()))
                    .headless(true)
                    .build()
                    .unwrap(),
            )
            .unwrap();
            handles.push(chrome);
        }
    }

    #[test]
    fn test_temporary_user_data_dir_is_removed_automatically() {
        setup();

        let options = LaunchOptions::default_builder().build().unwrap();

        // Ensure we did not pass an explicit user_data_dir
        let temp_dir = options.user_data_dir.clone();
        assert_eq!(None, temp_dir);

        let user_data_dir = {
            let _chrome = &mut super::Process::new(options).unwrap();

            ForTesting::USER_DATA_DIR.with(|dir| dir.borrow_mut().take())
        };

        match user_data_dir {
            Some(temp_path) => {
                let user_data_dir_exists = std::path::Path::new(&temp_path).is_dir();
                assert!(!user_data_dir_exists);
            }
            None => panic!("No user data dir was created"),
        }
    }
}
