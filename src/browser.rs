use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;

use failure::{Error, Fail};
use log::*;
use serde;

use crate::cdtp::{self, Event};
use crate::cdtp::target;
use crate::cdtp::target::methods::GetTargets;
use crate::cdtp::target::methods::SetDiscoverTargets;
use crate::logging;
pub use crate::process::LaunchOptions;
use crate::process::Process;
use crate::tab::Tab;
use crate::transport::Transport;
use crate::waiting_call_registry::WaitingCallRegistry;
use crate::web_socket_connection::WebSocketConnection;
use crate::helpers::{WaitOptions, wait_for};

pub struct Browser {
    pub process: Process,
    transport: Arc<Transport>,
    // TODO: surely doesn't need to be behind mutex for reads!
    tabs: Arc<Mutex<Vec<Arc<Tab>>>>,
}


impl Browser {
    pub fn new(launch_options: LaunchOptions) -> Result<Self, Error> {

        let process = Process::new(launch_options)?;

        let transport = Arc::new(Transport::new(process.debug_ws_url.clone())?);

        let tabs = Arc::new(Mutex::new(vec![]));

        let mut browser = Browser {
            process,
            tabs,
            transport,
        };

        let incoming_events_rx = browser.transport.listen_to_browser_events();
        browser.handle_incoming_messages(incoming_events_rx);

        // so we get events like 'targetCreated' and 'targetDestroyed'
        browser.call_method(SetDiscoverTargets { discover: true })?;

        Ok(browser)
    }

    pub fn wait_for_initial_tab(&self) -> Result<Arc<Tab>, Error> {
        wait_for(||{
            self.tabs.lock().unwrap().first().map(|tab| Arc::clone(tab))
        }, WaitOptions { timeout_ms: 300, sleep_ms: 10 })
    }

    pub fn new_tab(&self) -> Result<Arc<Tab>, Error> {
        let create_target = target::methods::CreateTarget {
            url: "about:blank",
            width: None,
            height: None,
            browser_context_id: None,
            enable_begin_frame_control: None,
        };

        let target_id = self.call_method(create_target)?.target_id;
        let new_tab = Arc::new(Tab::new(target_id, Arc::clone(&self.transport))?);

        self.add_tab(Arc::clone(&new_tab));

        Ok(new_tab)
    }

    fn add_tab(&self, tab: Arc<Tab>) {
        let mut tabs = self.tabs.lock().unwrap();
        tabs.push(tab);
    }

    // TODO: rename cdtp to protocol
    fn handle_incoming_messages(&self, events_rx: mpsc::Receiver<Event>) {
        let tabs = Arc::clone(&self.tabs);
        let transport = Arc::clone(&self.transport);

        std::thread::spawn(move || {
            for event in events_rx {
                match event {
                    Event::TargetCreated(ev) => {
                        trace!("Target created: {:?}", ev.params.target_info);
                        if ev.params.target_info.target_type == "page" {
                            let target_id = ev.params.target_info.target_id;
                            let new_tab = Arc::new(Tab::new(target_id, Arc::clone(&transport)).unwrap());
                            tabs.lock().unwrap().push(new_tab);
                        }
                    }
                    Event::TargetInfoChanged(ev) => {}
                    Event::TargetDestroyed(_) => {}
                    _ => {}
                }
            }
        });
    }

    pub fn call_method<C>(&self, method: C) -> Result<C::ReturnObject, Error>
        where C: cdtp::Method + serde::Serialize {
        self.transport.call_method(method)
    }
}

fn try_out_browser() -> Result<(), Error> {
    let mut browser = Browser::new(LaunchOptions { headless: true, ..Default::default() })?;

    let method = GetTargets {};
    let targets = browser.call_method(method)?.target_infos;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://wikipedia.org")?;
    std::thread::sleep_ms(4000);
    Ok(())
}


#[test]
fn browser_basic_test() {
    logging::enable_logging();
    try_out_browser().expect("returned error");
}



#[test]
fn ctrlc_chrome() {
    logging::enable_logging();
    let mut browser = Browser::new(LaunchOptions { headless: false, ..Default::default() }).unwrap();
    std::thread::sleep_ms(40_000);
}

// things to test:
// chrome comes with one target there by default.




