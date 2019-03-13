use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use failure::Error;
use log::*;
use which::which;

use serde;

use crate::protocol::browser::methods::GetVersion;
pub use crate::protocol::browser::methods::VersionInformationReturnObject;
use crate::protocol::target::methods::{CreateTarget, SetDiscoverTargets};
use crate::protocol::{self, Event};
use crate::util;

pub use process::LaunchOptionsBuilder;
use process::{LaunchOptions, Process};
use std::time::Duration;
pub use tab::Tab;
use transport::Transport;

mod fetcher;
mod process;
pub mod tab;
mod transport;

/// A handle to an instance of Chrome / Chromium, which wraps a WebSocket connection to its debugging port.
///
///
/// Most of your actual "driving" (e.g. clicking, typing, navigating) will be via instances of [Tab](../tab/struct.Tab.html), which are accessible via methods such as `get_tabs`.
///
/// With the default [LaunchOptions](../process/LaunchOptions.struct.html), we will automatically
/// download a revision of Chromium that has a compatible API into your `$XDG_DATA_DIR`. Alternatively,
/// you can specify your own path to a binary, or make use of the `default_executable` function to use
///  your already-installed copy of Chrome.
///
/// ```rust
/// # use failure::Error;
/// # fn main() -> Result<(), Error> {
/// #
/// use headless_chrome::{Browser, browser::default_executable, LaunchOptionsBuilder};
/// let browser = Browser::new(LaunchOptionsBuilder::default().path(Some(default_executable().unwrap())).build().unwrap())?;
/// let first_tab = browser.wait_for_initial_tab()?;
/// assert_eq!("about:blank", first_tab.get_url());
/// #
/// # Ok(())
/// # }
/// ```
///
/// While the Chrome DevTools Protocl (CDTP) does define some methods in a
/// ["Browser" domain](https://chromedevtools.github.io/devtools-protocol/tot/Browser)
/// (such as for resizing the window in non-headless mode), we currently don't implement those.
pub struct Browser {
    _process: Process,
    transport: Arc<Transport>,
    tabs: Arc<Mutex<Vec<Arc<Tab>>>>,
}

impl Browser {
    /// Launch a new Chrome browser.
    ///
    /// The browser will have its user data (aka "profile") directory stored in a temporary directory.
    /// The browser process will be killed when this struct is dropped.
    pub fn new(launch_options: LaunchOptions) -> Result<Self, Error> {
        let process = Process::new(launch_options)?;

        let transport = Arc::new(Transport::new(process.debug_ws_url.clone())?);

        trace!("created transport");

        let tabs = Arc::new(Mutex::new(vec![]));

        let browser = Self {
            _process: process,
            tabs,
            transport,
        };

        let incoming_events_rx = browser.transport.listen_to_browser_events();
        browser.handle_browser_level_events(incoming_events_rx);
        trace!("created browser event listener");

        // so we get events like 'targetCreated' and 'targetDestroyed'
        trace!("Calling set discover");
        browser.call_method(SetDiscoverTargets { discover: true })?;

        browser.wait_for_initial_tab()?;

        Ok(browser)
    }

    /// The tabs are behind an `Arc` and `Mutex` because they're accessible from multiple threads
    /// (including the one that handles incoming protocol events about new or changed tabs).
    pub fn get_tabs(&self) -> Arc<Mutex<Vec<Arc<Tab>>>> {
        Arc::clone(&self.tabs)
    }

    /// Chrome always launches with at least one tab. The reason we have to 'wait' is because information
    /// about that tab isn't available *immediately* after starting the process. Tabs are behind `Arc`s
    /// because they each have their own thread which handles events and method responses directed to them.
    pub fn wait_for_initial_tab(&self) -> Result<Arc<Tab>, Error> {
        util::Wait::default()
            .until(|| self.tabs.lock().unwrap().first().map(|tab| Arc::clone(tab)))
            .map_err(|e| e.into())
    }

    /// Create a new tab and return a handle to it.
    ///
    /// ```rust
    /// # use failure::Error;
    /// # fn main() -> Result<(), Error> {
    /// #
    /// # use headless_chrome::{Browser, browser::default_executable, LaunchOptionsBuilder};
    /// # let browser = Browser::new(LaunchOptionsBuilder::default().path(Some(default_executable().unwrap())).build().unwrap())?;
    /// let first_tab = browser.wait_for_initial_tab()?;
    /// let new_tab = browser.new_tab()?;
    /// let num_tabs = browser.get_tabs().lock().unwrap().len();
    /// assert_eq!(2, num_tabs);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Currently does not support creating the tab in a new "browser context", aka an incognito
    /// window.
    pub fn new_tab(&self) -> Result<Arc<Tab>, Error> {
        let create_target = CreateTarget {
            url: "about:blank",
            width: None,
            height: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
        };

        let target_id = self.call_method(create_target)?.target_id;

        util::Wait::default()
            .until(|| {
                let tabs = self.tabs.lock().unwrap();
                tabs.iter()
                    .find(|tab| *tab.get_target_id() == target_id)
                    .map(|tab_ref| Arc::clone(tab_ref))
            })
            .map_err(|e| e.into())
    }

    /// Get version information
    ///
    /// ```rust
    /// # use failure::Error;
    /// # fn main() -> Result<(), Error> {
    /// #
    /// # use headless_chrome::{Browser, browser::default_executable, LaunchOptionsBuilder};
    /// # let browser = Browser::new(LaunchOptionsBuilder::default().path(Some(default_executable().unwrap())).build().unwrap())?;
    /// let version_info = browser.get_version()?;
    /// println!("User-Agent is `{}`", version_info.user_agent);
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_version(&self) -> Result<VersionInformationReturnObject, Error> {
        self.call_method(GetVersion {})
    }

    fn handle_browser_level_events(&self, events_rx: mpsc::Receiver<Event>) {
        let tabs = Arc::clone(&self.tabs);
        let transport = Arc::clone(&self.transport);

        std::thread::spawn(move || {
            trace!("Starting browser's event handling loop");
            loop {
                match events_rx.recv_timeout(Duration::from_millis(20_000)) {
                    Err(_) => {
                        break;
                    }
                    Ok(event) => {
                        match event {
                            Event::TargetCreated(ev) => {
                                let target_info = ev.params.target_info;
                                trace!("Target created: {:?}", target_info);
                                if target_info.target_type.is_page() {
                                    match Tab::new(target_info, Arc::clone(&transport)) {
                                        Ok(new_tab) => {
                                            tabs.lock().unwrap().push(Arc::new(new_tab));
                                        }
                                        Err(_tab_creation_err) => {
                                            error!("Failed to create a handle to new tab");
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
                                    let updated_tab = locked_tabs
                                        .iter()
                                        .find(|tab| *tab.get_target_id() == target_info.target_id)
                                        .expect("got TargetInfoChanged event about a tab not in our list");
                                    updated_tab.update_target_info(target_info);
                                }
                            }
                            Event::TargetDestroyed(ev) => {
                                trace!("Target destroyed: {:?}", ev.params.target_id);
                            }
                            _ => {}
                        }
                    }
                }
            }
            trace!("Finished browser's event handling loop");
        });
    }

    /// Call a browser method.
    ///
    /// See the `cdtp` module documentation for available methods.
    fn call_method<C>(&self, method: C) -> Result<C::ReturnObject, Error>
    where
        C: protocol::Method + serde::Serialize,
    {
        self.transport.call_method_on_browser(method)
    }

    #[allow(dead_code)]
    #[cfg(test)]
    pub(crate) fn process(&self) -> &Process {
        #[allow(clippy::used_underscore_binding)]
        &self._process
    }
}

pub fn default_executable() -> Result<std::path::PathBuf, String> {
    // TODO Look at $BROWSER and if it points to a chrome binary
    // $BROWSER may also provide default arguments, which we may
    // or may not override later on.

    for app in &["google-chrome-stable", "chromium"] {
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
        if let Some(path) = get_chrome_path_from_registry() {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    Err("Could not auto detect a chrome executable".to_string())
}
