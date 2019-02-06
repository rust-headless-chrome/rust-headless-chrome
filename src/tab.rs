use std::cell::RefCell;

use log::*;
use failure::{Error, Fail};
use serde;

use crate::cdtp::dom;
use crate::cdtp::input;
use crate::cdtp::page::methods::Navigate;
use crate::page_session::PageSession;
use crate::element::Element;
use crate::keys;
use crate::point::Point;
use crate::cdtp;

pub type SessionReference = RefCell<PageSession>;

pub struct Tab {
    pub page_session: SessionReference,
}

#[derive(Debug, Fail)]
#[fail(display = "No element found for selector: {}", selector)]
pub struct NoElementFound {
    selector: String
}
#[derive(Debug, Fail)]
#[fail(display = "Navigate failed: {}", error_text)]
pub struct NavigationFailed {
    error_text: String
}
#[derive(Debug, Fail)]
#[fail(display = "Navigate timed out")]
pub struct NavigationTimedOut {}

impl Tab {
    // TODO: error handling (e.g. error_text: Some("net::ERR_CONNECTION_RESET"))
    pub fn call_method<C>(&self, method: C) -> Result<C::ReturnObject, Error>
        where C: cdtp::Method + serde::Serialize
    {
        let mut session = self.page_session.borrow_mut();
        session.call(method)
    }

    pub fn wait_until_navigated(&self) -> Result<(), Error> {
        let session = self.page_session.borrow_mut();
        trace!("waiting to start navigating");
        // wait for navigating to go to true

        let time_before = std::time::SystemTime::now();

        let timed_out = || {
            let elapsed_seconds = time_before
                .elapsed()
                .expect("serious problems with your clock bro")
                .as_secs();
            elapsed_seconds > 10
        };

        loop {
            if timed_out() {
                return Err(NavigationTimedOut {}.into());
            }
            if *session.navigating.lock().unwrap() {
                break;
            }
        }
        trace!("started navigating");

        // wait for navigating to go to false
        loop {
            if timed_out() {
                return Err(NavigationTimedOut {}.into());
            }
            if !*session.navigating.lock().unwrap() {
                break;
            }
        }
        Ok(())
    }

    pub fn navigate_to(&self, url: &str) -> Result<(), Error> {
        let return_object = self.call_method(Navigate { url })?;
        if let Some(error_text) = return_object.error_text {
            return Err(NavigationFailed { error_text }.into());
        }

        self.wait_until_navigated()?;

        Ok(())
    }

    // TODO: have this return a 'can't find element' error when selector returns nothing
    pub fn find_element(&self, selector: &str) -> Result<Element, Error> {
        let node_id = {
            let mut session = self.page_session.borrow_mut();
            // TODO: just do this once.
            let root_node_id = session.call(dom::methods::GetDocument {
                depth: Some(0),
                pierce: Some(false),
            })?.root.node_id;

            session.call(dom::methods::QuerySelector {
                node_id: root_node_id,
                selector,
            })?.node_id
        };

        if node_id == 0 {
            return Err(NoElementFound { selector: selector.to_string() }.into());
        }

        dbg!(node_id);

        let backend_node_id = self.describe_node(node_id)?.backend_node_id;

        dbg!(backend_node_id);

        let remote_object_id = {
            let mut session = self.page_session.borrow_mut();
            let object = session.call(dom::methods::ResolveNode {
                backend_node_id: Some(backend_node_id)
            })?.object;
            object.object_id.expect("couldn't find object ID")
        };
        Ok(Element {
            remote_object_id,
            backend_node_id,
            parent: &self,
        })
    }

    pub fn describe_node(&self, node_id: dom::NodeId) -> Result<dom::Node, Error> {
        let mut session = self.page_session.borrow_mut();
        let node = session.call(dom::methods::DescribeNode {
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
        let mut session = self.page_session.borrow_mut();

        // TODO: send code and other parts of the def?
        session.call(input::methods::DispatchKeyEvent {
            event_type: "keyDown",
            key: definition.key,
            text: definition.text,
        })?;
        session.call(input::methods::DispatchKeyEvent {
            event_type: "keyUp",
            key: definition.key,
            text: definition.text,
        })?;
        Ok(())
    }


    pub fn click_point(&self, point: Point) -> Result<(), Error> {
        trace!("clicking point: {:?}", point);

        let mut session = self.page_session.borrow_mut();

        session.call(input::methods::DispatchMouseEvent {
            event_type: "mouseMoved",
            x: point.x,
            y: point.y,
            ..Default::default()
        })?;
        std::thread::sleep_ms(100);
        session.call(input::methods::DispatchMouseEvent {
            event_type: "mousePressed",
            x: point.x,
            y: point.y,
            button: Some("left"),
            click_count: Some(1)
        })?;
        std::thread::sleep_ms(100);
        session.call(input::methods::DispatchMouseEvent {
            event_type: "mouseReleased",
            x: point.x,
            y: point.y,
            button: Some("left"),
            click_count: Some(1)
        })?;
        std::thread::sleep_ms(100);
        Ok(())
    }
}