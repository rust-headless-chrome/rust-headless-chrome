use failure::Error;

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


pub struct Element<'a> {
    pub remote_object_id: String,
    pub backend_node_id: dom::NodeId,
    pub parent: &'a tab::Tab,
}

impl<'a> Element<'a> {
    pub fn click(&self) -> Result<(), Error> {
        let midpoint = self.get_midpoint()?;
        self.parent.click_point(midpoint)?;
        Ok(())
    }

    pub fn focus(&self) -> Result<(), Error> {
        self.parent.call_method(dom::methods::Focus {
            backend_node_id: Some(self.backend_node_id),
            ..Default::default()
        })?;
        Ok(())
    }

    pub fn get_description(&self) -> Result<dom::Node, Error> {
        let mut session = self.parent.page_session.borrow_mut();
        let node = session.call(dom::methods::DescribeNode {
            node_id: None,
            backend_node_id: Some(self.backend_node_id),
            depth: Some(100),
        })?.node;
        Ok(node)
    }

    pub fn get_attributes(&self) -> Result<(), Error> {
        let description = self.get_description()?;
        dbg!(description.attributes);
        Ok(())
    }

    pub fn get_midpoint(&self) -> Result<Point, Error> {
        let mut session = self.parent.page_session.borrow_mut();

        let return_object = session.call(dom::methods::GetContentQuads {
            node_id: None,
            backend_node_id: Some(self.backend_node_id),
            object_id: None,
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
