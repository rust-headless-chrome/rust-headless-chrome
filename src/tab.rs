use std::sync::Arc;

use failure::{Error, Fail};
use log::*;
use serde;

use crate::cdtp;
use crate::cdtp::dom;
use crate::cdtp::input;
use crate::cdtp::page;
use crate::cdtp::page::methods::Navigate;
use crate::cdtp::target;
use crate::cdtp::target::TargetId;
use crate::element::Element;
use crate::helpers::{wait_for, WaitOptions};
use crate::keys;
use crate::point::Point;
use crate::transport;
use crate::transport::Transport;
use crate::cdtp::Event;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use crate::cdtp::target::TargetInfo;

pub struct Tab {
    target_id: TargetId,
    transport: Arc<Transport>,
    session_id: transport::SessionId,
    navigating: Arc<AtomicBool>,
    target_info: Arc<Mutex<TargetInfo>>
}

#[derive(Debug, Fail)]
#[fail(display = "No element found for selector: {}", selector)]
pub struct NoElementFound {
    selector: String
}

#[derive(Debug, Fail)]
#[fail(display = "Timed out waiting for something")]
pub struct TimeOut {}

#[derive(Debug, Fail)]
#[fail(display = "Navigate failed: {}", error_text)]
pub struct NavigationFailed {
    error_text: String
}

#[derive(Debug, Fail)]
#[fail(display = "Navigate timed out")]
pub struct NavigationTimedOut {}

impl<'a> Tab {
    pub fn new(target_info: TargetInfo, transport: Arc<Transport>) -> Result<Self, Error> {
        // TODO: attach to target!!!
        let target_id = target_info.target_id.clone();

        let session_id = transport.call_method(target::methods::AttachToTarget {
            target_id: &target_id,
            flatten: None,
        })?.session_id;

        debug!("New tab attached with session ID: {}", session_id);

        let target_info_mutex = Arc::new(Mutex::new(target_info));

        let tab = Tab {
            target_id,
            transport,
            session_id,
            navigating: Arc::new(AtomicBool::new(false)),
            target_info: target_info_mutex
        };

        tab.call_method(page::methods::Enable {})?;
        tab.call_method(page::methods::SetLifecycleEventsEnabled { enabled: true })?;

        tab.start_event_handler_thread();

        Ok(tab)
    }

    pub fn update_target_info(&self, target_info: TargetInfo) {
        let mut info = self.target_info.lock().unwrap();
        *info = target_info;
    }

    pub fn get_target_id(&self) -> &TargetId {
        &self.target_id
    }

    // TODO: kinda sucks that I do an allocation here
    pub fn get_url(&self) -> String {
        let info = self.target_info.lock().unwrap();
        info.url.clone()
    }

    fn start_event_handler_thread(&self) {
        let incoming_events_rx = self.transport.listen_to_target_events(self.session_id.clone());
        let navigating = Arc::clone(&self.navigating);

        std::thread::spawn(move || {
            for event in incoming_events_rx {
                trace!("{:?}", &event);
                match event {
                    Event::LifecycleEvent(lifecycle_event) => {
//                        if lifecycle_event.params.frame_id == main_frame_id {
                        match lifecycle_event.params.name.as_ref() {
                            "networkAlmostIdle" => {
                                navigating.store(false, Ordering::SeqCst);
                            }
                            "init" => {
                                navigating.store(true, Ordering::SeqCst);
                            }
                            _ => {}
                        }
//                        }
                    }
                    _ => {}
                }
            }
        });
    }

    pub fn call_method<C>(&self, method: C) -> Result<C::ReturnObject, Error>
        where C: cdtp::Method + serde::Serialize + std::fmt::Debug {
        trace!("Calling method: {:?}", method);
        self.transport.call_method_on_target(&self.session_id, method)
    }

    pub fn wait_until_navigated(&self) -> Result<(), Error> {
        trace!("waiting to start navigating");
        // wait for navigating to go to true
        let navigating = Arc::clone(&self.navigating);
        wait_for(||{
            if navigating.load(Ordering::SeqCst) {
                Some(true)
            } else {
                None
            }
        }, WaitOptions { timeout_ms: 15_000, sleep_ms: 100 })?;
        debug!("A tab started navigating");

        wait_for(||{
            if navigating.load(Ordering::SeqCst) {
                None
            } else {
                Some(true)
            }
        }, WaitOptions { timeout_ms: 15_000, sleep_ms: 100 })?;
        debug!("A tab finished navigating");

        Ok(())
    }

    pub fn navigate_to(&self, url: &str) -> Result<(), Error> {
        let return_object = self.call_method(Navigate { url })?;
        if let Some(error_text) = return_object.error_text {
            return Err(NavigationFailed { error_text }.into());
        }

        // TODO: implement
//        self.wait_until_navigated()?;

        info!("Navigated a tab to {}", url);

        Ok(())
    }

    pub fn wait_for_element(&'a self, selector: &'a str) -> Result<Element<'a>, Error> {
        self.wait_for_element_with_custom_timeout(selector, 15_000)
    }

    pub fn wait_for_element_with_custom_timeout(&'a self, selector: &'a str,
                                                timeout_ms: u64) -> Result<Element<'a>, Error> {
        debug!("Waiting for element with selector: {}", selector);
        wait_for(|| {
            // TODO: there must be a pattern to replace these nested ifs
            if let Ok(element) = self.find_element(selector) {
                if element.get_midpoint().is_ok() {
                    Some(element)
                } else {
                    None
                }
            } else {
                None
            }
        }, WaitOptions { timeout_ms, sleep_ms: 1000 })
    }

    // TODO: have this return a 'can't find element' error when selector returns nothing
    pub fn find_element(&'a self, selector: &'a str) -> Result<Element<'a>, Error> {
        trace!("Looking up element via selector: {}", selector);

        let node_id = {
            // TODO: just do this once.
            let root_node_id = self.call_method(dom::methods::GetDocument {
                depth: Some(0),
                pierce: Some(false),
            })?.root.node_id;

            self.call_method(dom::methods::QuerySelector {
                node_id: root_node_id,
                selector,
            })?.node_id
        };

        if node_id == 0 {
            return Err(NoElementFound { selector: selector.to_string() }.into());
        }

        let backend_node_id = self.describe_node(node_id)?.backend_node_id;

        let remote_object_id = {
            let object = self.call_method(dom::methods::ResolveNode {
                backend_node_id: Some(backend_node_id)
            })?.object;
            object.object_id.expect("couldn't find object ID")
        };
        Ok(Element {
            remote_object_id,
            backend_node_id,
            parent: &self,
            found_via_selector: selector,
        })
    }

    pub fn describe_node(&self, node_id: dom::NodeId) -> Result<dom::Node, Error> {
        let node = self.call_method(dom::methods::DescribeNode {
            node_id: Some(node_id),
            backend_node_id: None,
            depth: Some(100),
        })?.node;
        Ok(node)
    }

    pub fn type_str(&self, string_to_type: &str) -> Result<(), Error> {
        for c in string_to_type.split("") {
            // split call above will have empty string at start and end which we won't type
            if c == "" {
                continue;
            }
            self.press_key(c)?;
        }
        Ok(())
    }

    pub fn press_key(&self, key: &str) -> Result<(), Error> {
        let definition = keys::get_key_definition(key)?;

        // TODO: send code and other parts of the def?
        self.call_method(input::methods::DispatchKeyEvent {
            event_type: "keyDown",
            key: definition.key,
            text: definition.text,
        })?;
        self.call_method(input::methods::DispatchKeyEvent {
            event_type: "keyUp",
            key: definition.key,
            text: definition.text,
        })?;
        Ok(())
    }


    pub fn click_point(&self, point: Point) -> Result<(), Error> {
        trace!("Clicking point: {:?}", point);
        if point.x == 0.0 && point.y == 0.0 {
            warn!("Midpoint of element shouldn't be 0,0. Something is probably wrong.")
        }


        self.call_method(input::methods::DispatchMouseEvent {
            event_type: "mouseMoved",
            x: point.x,
            y: point.y,
            ..Default::default()
        })?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.call_method(input::methods::DispatchMouseEvent {
            event_type: "mousePressed",
            x: point.x,
            y: point.y,
            button: Some("left"),
            click_count: Some(1),
        })?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.call_method(input::methods::DispatchMouseEvent {
            event_type: "mouseReleased",
            x: point.x,
            y: point.y,
            button: Some("left"),
            click_count: Some(1),
        })?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }
}
