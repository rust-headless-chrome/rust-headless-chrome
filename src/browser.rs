use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Mutex;

use failure::Error;
use log::*;
use serde;

use crate::cdtp::{self, Event};
use crate::cdtp::target::methods::SetDiscoverTargets;
use crate::helpers::{wait_for, WaitOptions};
pub use crate::process::LaunchOptions;
use crate::process::Process;
use crate::tab::Tab;
use crate::transport::Transport;

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

        let browser = Browser {
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

    pub fn get_tabs(&self) -> Arc<Mutex<Vec<Arc<Tab>>>> {
        Arc::clone(&self.tabs)
    }

    pub fn wait_for_initial_tab(&self) -> Result<Arc<Tab>, Error> {
        wait_for(|| {
            self.tabs.lock().unwrap().first().map(|tab| Arc::clone(tab))
        }, WaitOptions { timeout_ms: 300, sleep_ms: 10 })
    }

//    pub fn new_tab(&self) -> Result<Arc<Tab>, Error> {
//        let create_target = target::methods::CreateTarget {
//            url: "about:blank",
//            width: None,
//            height: None,
//            browser_context_id: None,
//            enable_begin_frame_control: None,
//        };
//
////        let target_id = self.call_method(create_target)?.target_id;
////        let new_tab = Arc::new(Tab::new(target_id, Arc::clone(&self.transport))?);
////
////        self.add_tab(Arc::clone(&new_tab));
//
//        Ok(new_tab)
//    }

    // TODO: rename cdtp to protocol
    fn handle_incoming_messages(&self, events_rx: mpsc::Receiver<Event>) {
        let tabs = Arc::clone(&self.tabs);
        let transport = Arc::clone(&self.transport);

        std::thread::spawn(move || {
            for event in events_rx {
                match event {
                    Event::TargetCreated(ev) => {
                        let target_info = ev.params.target_info;
                        trace!("Target created: {:?}", target_info);
                        if target_info.target_type == "page" {
                            let new_tab = Arc::new(Tab::new(target_info, Arc::clone(&transport)).unwrap());
                            tabs.lock().unwrap().push(new_tab);
                        }
                    }
                    Event::TargetInfoChanged(ev) => {
                        let target_info = ev.params.target_info;
                        trace!("Target info changed: {:?}", target_info);
                        if target_info.target_type == "page" {
                            let locked_tabs = tabs.lock().unwrap();
                            dbg!(locked_tabs.len());
                            let updated_tab = locked_tabs.iter().find(|tab| {
                                *tab.get_target_id() == target_info.target_id
                            }).expect("got TargetInfoChanged event about a tab not in our list");
                            updated_tab.update_target_info(target_info);
                        }
                    }
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

#[test]
fn browser_basic_test() {
    use crate::logging;
    fn try_out_browser() -> Result<(), Error> {
        let mut browser = Browser::new(LaunchOptions { headless: true, ..Default::default() })?;

        let method = GetTargets {};
        let targets = browser.call_method(method)?.target_infos;
        let tab = browser.wait_for_initial_tab()?;
        tab.navigate_to("https://wikipedia.org")?;
        std::thread::sleep_ms(4000);
        Ok(())
    }
    logging::enable_logging();
    try_out_browser().expect("returned error");
}


#[test]
fn ctrlc_chrome() {
    use crate::logging;
    logging::enable_logging();
    let mut browser = Browser::new(LaunchOptions { headless: false, ..Default::default() }).unwrap();
    std::thread::sleep_ms(40_000);
}

// things to test:
// chrome comes with one target there by default.




