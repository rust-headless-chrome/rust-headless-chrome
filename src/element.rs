use std::cell::RefCell;
use std::rc::Rc;

use crate::errors::*;
use crate::page_session::PageSession;
use crate::point::Point;
use crate::cdtp::dom;
use crate::element;
use crate::tab;


#[derive(Debug, Copy, Clone)]
pub struct ElementQuad {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}


pub struct Element {
    pub remote_object_id: String,
    pub backend_node_id: dom::NodeId,
    pub session: tab::SessionReference
}

impl Element {

    pub fn click(&self) -> Result<()> {
        let midpoint = self.get_midpoint()?;
        self.session.borrow_mut().click_point(midpoint);
        Ok(())
    }

    pub fn get_midpoint(&self) -> Result<Point> {
        let mut session = self.session.borrow_mut();

        let return_object = session.call(dom::methods::GetContentQuads {
            node_id: None,
            backend_node_id: Some(self.backend_node_id),
            object_id: None
        })?;

        let raw_quad = return_object.quads.first().unwrap();

        let input_quad = element::ElementQuad {
            top_left: Point { x: raw_quad[0], y: raw_quad[1] },
            top_right: Point { x: raw_quad[2], y: raw_quad[3] },
            bottom_right: Point { x: raw_quad[4], y: raw_quad[5] },
            bottom_left: Point { x: raw_quad[6], y: raw_quad[7] },
        };

        Ok((input_quad.bottom_right + input_quad.top_left) / 2.0)
    }
}
