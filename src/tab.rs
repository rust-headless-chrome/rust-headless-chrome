use std::cell::RefCell;

use log::*;

use crate::cdtp::dom;
use crate::cdtp::input;
use crate::cdtp::page::methods::Navigate;
use crate::errors::*;
use crate::page_session::PageSession;
use crate::element::Element;
use crate::point::Point;

pub type SessionReference = RefCell<PageSession>;

pub struct Tab {
    pub page_session: SessionReference,
}

impl Tab {
    // TODO: error handling
    pub fn navigate_to(&self, url: &str) -> Result<()> {
        let mut session = self.page_session.borrow_mut();
        let nav_result = session.call(Navigate { url: url.to_string() })?;

        // TODO: at least add a timeout for these loops. it's a disaster waiting to happen!

        trace!("waiting to start navigating");
        // wait for navigating to go to true
        loop {
            if (*session.navigating.lock().unwrap()) {
                break;
            }
        }
        trace!("started navigating");

        // wait for navigating to go to false
        loop {
            if (!*session.navigating.lock().unwrap()) {
                break;
            }
        }

        trace!("done navigating");
        Ok(())
    }

    pub fn find_element(&self, selector: &str) -> Result<Element> {
        let node_id = {
            let mut session = self.page_session.borrow_mut();
            // TODO: just do this once.
            let root_node_id = session.call(dom::methods::GetDocument {
                depth: Some(0),
                pierce: Some(false),
            })?.root.node_id;

            session.call(dom::methods::QuerySelector {
                // TODO: this too risky? use getDocument instead?
                node_id: root_node_id,
                selector: selector.to_string(),
            })?.node_id
        };

        let backend_node_id = self.describe_node(node_id)?.backend_node_id;

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
            parent: &self
        })
    }

    pub fn describe_node(&self, node_id: dom::NodeId) -> Result<dom::Node> {
        let mut session = self.page_session.borrow_mut();
        let node = session.call(dom::methods::DescribeNode {
            node_id: Some(node_id),
            backend_node_id: None,
            depth: Some(100),
        })?.node;
        Ok(node)
    }

    // TODO: have a 'key description' struct
    pub fn press_key(&self, key: &str, text: &str) {
        let mut session = self.page_session.borrow_mut();

        session.call(input::methods::DispatchKeyEvent {
            event_type: "keyDown".to_string(),
            key: key.to_string(),
            text: text.to_string(),
        });
        session.call(input::methods::DispatchKeyEvent {
            event_type: "keyUp".to_string(),
            key: key.to_string(),
            text: text.to_string(),
        });
    }

    pub fn click_point(&self, point: Point) -> Result<()> {
        let mut session = self.page_session.borrow_mut();

        session.call(input::methods::DispatchMouseEvent {
            event_type: "mouseMoved".to_string(),
            x: point.x,
            y: point.y,
        });
        session.call(input::methods::DispatchMouseEvent {
            event_type: "mousePressed".to_string(),
            x: point.x,
            y: point.y,
        });
        session.call(input::methods::DispatchMouseEvent {
            event_type: "mouseReleased".to_string(),
            x: point.x,
            y: point.y,
        });
        Ok(())
    }
}