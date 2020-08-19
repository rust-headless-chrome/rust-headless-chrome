use std::sync::mpsc;
use std::sync::mpsc::{RecvTimeoutError, TryRecvError};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use failure::Fallible;
use log::*;
use serde;

use process::Process;
pub use process::{LaunchOptions, LaunchOptionsBuilder};
pub use tab::Tab;
pub use transport::ConnectionClosed;
use transport::Transport;
use which::which;

use crate::browser::context::Context;
use crate::protocol::browser::methods::GetVersion;
pub use crate::protocol::browser::methods::VersionInformationReturnObject;
use crate::protocol::target::methods::{CreateTarget, SetDiscoverTargets};
use crate::protocol::{self, Event};
use crate::util;

#[cfg(feature = "fetch")]
pub use fetcher::FetcherOptions;

pub mod context;
#[cfg(feature = "fetch")]
mod fetcher;
mod process;
pub mod tab;
mod transport;

/// A handle to an instance of Chrome / Chromium, which wraps a WebSocket connection to its debugging port.
///
///
/// Most of your actual "driving" (e.g. clicking, typing, navigating) will be via instances of [Tab](../tab/struct.Tab.html), which are accessible via methods such as `get_tabs`.
///
/// A Browser can either manage its own Chrome process or connect to a remote one.
///
/// `Browser::default().unwrap()` will return a headless instance of whatever browser can be found using
/// `default_executable`, which will search on your PATH for relevant binaries or use the path
/// specified in the `CHROME` env var.
///
/// You can use [LaunchOptions](../process/LaunchOptions.struct.html) to automatically
/// download a revision of Chromium that has a compatible API into your `$XDG_DATA_DIR`. Alternatively,
/// you can specify your own path to a binary, or make use of the `default_executable` function to use
///  your already-installed copy of Chrome.
///
/// Option 1: Managing a Chrome process
/// ```rust
/// # use failure::Fallible;
/// # fn main() -> Fallible<()> {
/// #
/// use headless_chrome::Browser;
/// let browser = Browser::default()?;
/// let first_tab = browser.wait_for_initial_tab()?;
/// assert_eq!("about:blank", first_tab.get_url());
/// #
/// # Ok(())
/// # }
/// ```
///
/// Option 2: Connecting to a remote Chrome service
/// - see /examples/print_to_pdf.rs for a working example
///
///
/// While the Chrome DevTools Protocl (CDTP) does define some methods in a
/// ["Browser" domain](https://chromedevtools.github.io/devtools-protocol/tot/Browser)
/// (such as for resizing the window in non-headless mode), we currently don't implement those.
pub struct Browser {
    process: Option<Process>,
    transport: Arc<Transport>,
    tabs: Arc<Mutex<Vec<Arc<Tab>>>>,
    loop_shutdown_tx: mpsc::Sender<()>,
}

impl Browser {
    /// Launch a new Chrome browser.
    ///
    /// The browser will have its user data (aka "profile") directory stored in a temporary directory.
    /// The browser process will be killed when this struct is dropped.
    pub fn new(launch_options: LaunchOptions) -> Fallible<Self> {
        let idle_browser_timeout = launch_options.idle_browser_timeout;
        let process = Process::new(launch_options)?;
        let process_id = process.get_id();

        let transport = Arc::new(Transport::new(
            process.debug_ws_url.clone(),
            Some(process_id),
            idle_browser_timeout,
        )?);

        Self::create_browser(Some(process), transport, idle_browser_timeout)
    }

    /// Calls [`new`] with options to launch a headless browser using whatever Chrome / Chromium
    /// binary can be found on the system.
    pub fn default() -> Fallible<Self> {
        let launch_options = LaunchOptions::default_builder()
            .path(Some(default_executable().unwrap()))
            .build()
            .unwrap();
        Ok(Self::new(launch_options).unwrap())
    }

    /// Allows you to drive an externally-launched Chrome process instead of launch one via [`new`].
    pub fn connect(debug_ws_url: String) -> Fallible<Self> {
        let transport = Arc::new(Transport::new(debug_ws_url, None, Duration::from_secs(30))?);
        trace!("created transport");

        Self::create_browser(None, transport, Duration::from_secs(30))
    }

    fn create_browser(
        process: Option<Process>,
        transport: Arc<Transport>,
        idle_browser_timeout: Duration,
    ) -> Fallible<Self> {
        let tabs = Arc::new(Mutex::new(vec![]));

        let (shutdown_tx, shutdown_rx) = mpsc::channel();

        let browser = Self {
            process,
            tabs,
            transport,
            loop_shutdown_tx: shutdown_tx,
        };

        let incoming_events_rx = browser.transport.listen_to_browser_events();

        browser.handle_browser_level_events(
            incoming_events_rx,
            browser.get_process_id(),
            shutdown_rx,
            idle_browser_timeout,
        );
        trace!("created browser event listener");

        // so we get events like 'targetCreated' and 'targetDestroyed'
        trace!("Calling set discover");
        browser.call_method(SetDiscoverTargets { discover: true })?;

        browser.wait_for_initial_tab()?;

        Ok(browser)
    }

    pub fn get_process_id(&self) -> Option<u32> {
        if let Some(process) = &self.process {
            Some(process.get_id())
        } else {
            None
        }
    }

    /// The tabs are behind an `Arc` and `Mutex` because they're accessible from multiple threads
    /// (including the one that handles incoming protocol events about new or changed tabs).
    pub fn get_tabs(&self) -> &Arc<Mutex<Vec<Arc<Tab>>>> {
        &self.tabs
    }

    /// Chrome always launches with at least one tab. The reason we have to 'wait' is because information
    /// about that tab isn't available *immediately* after starting the process. Tabs are behind `Arc`s
    /// because they each have their own thread which handles events and method responses directed to them.
    pub fn wait_for_initial_tab(&self) -> Fallible<Arc<Tab>> {
        util::Wait::with_timeout(Duration::from_secs(10))
            .until(|| self.tabs.lock().unwrap().first().map(|tab| Arc::clone(tab)))
            .map_err(Into::into)
    }

    /// Create a new tab and return a handle to it.
    ///
    /// If you want to specify its starting options, see `new_tab_with_options`.
    ///
    /// ```rust
    /// # use failure::Fallible;
    /// # fn main() -> Fallible<()> {
    /// #
    /// # use headless_chrome::Browser;
    /// # let browser = Browser::default()?;
    /// let first_tab = browser.wait_for_initial_tab()?;
    /// let new_tab = browser.new_tab()?;
    /// let num_tabs = browser.get_tabs().lock().unwrap().len();
    /// assert_eq!(2, num_tabs);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_tab(&self) -> Fallible<Arc<Tab>> {
        let default_blank_tab = CreateTarget {
            url: "about:blank",
            width: None,
            height: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
        };
        self.new_tab_with_options(default_blank_tab)
    }

    /// Create a new tab with a starting url, height / width, context ID and 'frame control'
    /// ```rust
    /// # use failure::Fallible;
    /// # fn main() -> Fallible<()> {
    /// #
    /// # use headless_chrome::{Browser, protocol::target::methods::CreateTarget};
    /// # let browser = Browser::default()?;
    ///    let new_tab = browser.new_tab_with_options(CreateTarget {
    ///    url: "chrome://version",
    ///    width: Some(1024),
    ///    height: Some(800),
    ///    browser_context_id: None,
    ///    enable_begin_frame_control: None,
    ///    })?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn new_tab_with_options(&self, create_target_params: CreateTarget) -> Fallible<Arc<Tab>> {
        let target_id = self.call_method(create_target_params)?.target_id;

        util::Wait::with_timeout(Duration::from_secs(20))
            .until(|| {
                let tabs = self.tabs.lock().unwrap();
                tabs.iter().find_map(|tab| {
                    if *tab.get_target_id() == target_id {
                        Some(tab.clone())
                    } else {
                        None
                    }
                })
            })
            .map_err(Into::into)
    }

    /// Creates the equivalent of a new incognito window, AKA a browser context
    pub fn new_context(&self) -> Fallible<context::Context> {
        debug!("Creating new browser context");
        let context_id = self
            .call_method(protocol::target::methods::CreateBrowserContext {})?
            .browser_context_id;
        debug!("Created new browser context: {:?}", context_id);
        Ok(Context::new(self, context_id))
    }

    /// Get version information
    ///
    /// ```rust
    /// # use failure::Fallible;
    /// # fn main() -> Fallible<()> {
    /// #
    /// # use headless_chrome::Browser;
    /// # let browser = Browser::default()?;
    /// let version_info = browser.get_version()?;
    /// println!("User-Agent is `{}`", version_info.user_agent);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_version(&self) -> Fallible<VersionInformationReturnObject> {
        self.call_method(GetVersion {})
    }

    fn handle_browser_level_events(
        &self,
        events_rx: mpsc::Receiver<Event>,
        process_id: Option<u32>,
        shutdown_rx: mpsc::Receiver<()>,
        idle_browser_timeout: Duration,
    ) {
        let tabs = Arc::clone(&self.tabs);
        let transport = Arc::clone(&self.transport);

        std::thread::spawn(move || {
            trace!("Starting browser's event handling loop");
            loop {
                match shutdown_rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        info!("Browser event loop received shutdown message");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }

                match events_rx.recv_timeout(idle_browser_timeout) {
                    Err(recv_timeout_error) => {
                        match recv_timeout_error {
                            RecvTimeoutError::Timeout => {
                                error!(
                                    "Got a timeout while listening for browser events (Chrome #{:?})",
                                    process_id
                                );
                            }
                            RecvTimeoutError::Disconnected => {
                                debug!(
                                    "Browser event sender disconnected while loop was waiting (Chrome #{:?})",
                                    process_id
                                );
                            }
                        }
                        break;
                    }
                    Ok(event) => match event {
                        Event::TargetCreated(ev) => {
                            let target_info = ev.params.target_info;
                            trace!("Creating target: {:?}", target_info);
                            if target_info.target_type.is_page() {
                                match Tab::new(target_info, Arc::clone(&transport)) {
                                    Ok(new_tab) => {
                                        tabs.lock().unwrap().push(Arc::new(new_tab));
                                    }
                                    Err(_tab_creation_err) => {
                                        info!("Failed to create a handle to new tab");
                                        break;
                                    }
                                }
                            }
                        }
                        Event::TargetInfoChanged(ev) => {
                            let target_info = ev.params.target_info;
                            trace!("Target info changed: {:?}", target_info);
                            if target_info.target_type.is_page() {
                                let locked_tabs = tabs.lock().unwrap();
                                if let Some(updated_tab) = locked_tabs
                                    .iter()
                                    .find(|tab| *tab.get_target_id() == target_info.target_id)
                                {
                                    updated_tab.update_target_info(target_info);
                                }
                            }
                        }
                        Event::TargetDestroyed(ev) => {
                            trace!("Target destroyed: {:?}", ev.params.target_id);
                            let mut locked_tabs = tabs.lock().unwrap();
                            let pos = locked_tabs
                                .iter()
                                .position(|tab| *tab.get_target_id() == ev.params.target_id);

                            if let Some(idx) = pos {
                                locked_tabs.remove(idx);
                            }
                        }
                        _ => {
                            let mut raw_event = format!("{:?}", event);
                            raw_event.truncate(50);
                            trace!("Unhandled event: {}", raw_event);
                        }
                    },
                }
            }
            info!("Finished browser's event handling loop");
        });
    }

    /// Call a browser method.
    ///
    /// See the `cdtp` module documentation for available methods.
    fn call_method<C>(&self, method: C) -> Fallible<C::ReturnObject>
    where
        C: protocol::Method + serde::Serialize,
    {
        self.transport.call_method_on_browser(method)
    }

    #[allow(dead_code)]
    #[cfg(test)]
    pub(crate) fn process(&self) -> Option<&Process> {
        #[allow(clippy::used_underscore_binding)]
        self.process.as_ref()
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        info!("Dropping browser");
        let _ = self.loop_shutdown_tx.send(());
        self.transport.shutdown();
    }
}

/// Returns the path to Chrome's executable.
///
/// If the `CHROME` environment variable is set, `default_executable` will
/// use it as the default path. Otherwise, the filenames `google-chrome-stable`
/// `chromium`, `chromium-browser`, `chrome` and `chrome-browser` are
/// searched for in standard places. If that fails,
/// `/Applications/Google Chrome.app/...` (on MacOS) or the registry (on Windows)
/// is consulted. If all of the above fail, an error is returned.
pub fn default_executable() -> Result<std::path::PathBuf, String> {
    if let Ok(path) = std::env::var("CHROME") {
        if std::path::Path::new(&path).exists() {
            return Ok(path.into());
        }
    }

    for app in &[
        "google-chrome-stable",
        "chromium",
        "chromium-browser",
        "chrome",
        "chrome-browser",
    ] {
        if let Ok(path) = which(app) {
            return Ok(path);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let default_paths = &["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"][..];
        for path in default_paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.into());
            }
        }
    }

    #[cfg(windows)]
    {
        use crate::browser::process::get_chrome_path_from_registry;

        if let Some(path) = get_chrome_path_from_registry() {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    Err("Could not auto detect a chrome executable".to_string())
}
