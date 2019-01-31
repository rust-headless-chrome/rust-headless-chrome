use std::cell::RefCell;

use log::*;

use crate::cdtp::dom;
use crate::cdtp::page::methods::Navigate;
use crate::errors::*;
use crate::page_session::PageSession;
use std::rc::Rc;
use crate::element::Element;

pub type SessionReference = Rc<RefCell<PageSession>>;

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

        let backend_node_id = {
            let mut session = self.page_session.borrow_mut();
            let node_description = session.describe_node(node_id)?;
            node_description.backend_node_id
        };

        let remote_object_id = {
            let mut session = self.page_session.borrow_mut();
            let object = session.call(dom::methods::ResolveNode {
                backend_node_id: Some(backend_node_id)
            })?.object;
            object.object_id.expect("couldn't find object ID")
        };
        Ok(Element {
            session: Rc::clone(&self.page_session),
            remote_object_id,
            backend_node_id
        })
    }

    pub fn describe_node(&self, node_id: dom::NodeId) -> Result<dom::Node> {
        let mut session = self.page_session.borrow_mut();
        let node = session.call(dom::methods::DescribeNode {
            node_id,
            depth: Some(100),
        })?.node;
        Ok(node)
    }
}