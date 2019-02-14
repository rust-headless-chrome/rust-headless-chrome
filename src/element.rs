use log::*;
use failure::Error;

use crate::point::Point;
use crate::cdtp::dom;
use crate::cdtp::runtime;
use crate::element;
use crate::tab;
use std::collections::HashMap;


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
    pub found_via_selector: &'a str,
}

impl<'a> Element<'a> {
    pub fn click(&self) -> Result<(), Error> {
        debug!("Clicking element found via {}", self.found_via_selector);

        let midpoint = self.get_midpoint()?;
        self.parent.click_point(midpoint)?;
        Ok(())
    }

    pub fn type_into(&self, text: &str) -> Result<(), Error> {
        self.click()?;

        debug!("Typing into element ( {} ): {}", self.found_via_selector, text);

        self.parent.type_str(text)?;

        Ok(())
    }

    pub fn call_js_fn(&self, function_declaration: &str) -> Result<runtime::methods::RemoteObject, Error> {
        let result = self.parent.call_method(runtime::methods::CallFunctionOn {
            object_id: &self.remote_object_id,
            function_declaration,
            return_by_value: false,
            generate_preview: true,
            silent: false,
        })?.result;

        Ok(result)
    }

    pub fn focus(&self) -> Result<(), Error> {
        self.parent.call_method(dom::methods::Focus {
            backend_node_id: Some(self.backend_node_id),
            ..Default::default()
        })?;
        Ok(())
    }

    pub fn get_description(&self) -> Result<dom::Node, Error> {
        let node = self.parent.call_method(dom::methods::DescribeNode {
            node_id: None,
            backend_node_id: Some(self.backend_node_id),
            depth: Some(100),
        })?.node;
        Ok(node)
    }

    pub fn set_input_files(&self, file_paths: &Vec<&str>) -> Result<(), Error> {
        self.parent.call_method(dom::methods::SetFileInputFiles {
            files: file_paths,
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_id: None,
        })?;
        Ok(())
    }

    pub fn get_attributes(&self) -> Result<Option<dom::NodeAttributes>, Error> {
        let description = self.get_description()?;
        Ok(description.attributes)
    }

    pub fn get_midpoint(&self) -> Result<Point, Error> {
        let return_object = self.parent.call_method(dom::methods::GetContentQuads {
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

    pub fn get_js_midpoint(&self) -> Result<Point, Error> {
        let result = self.call_js_fn("function(){ return this.getBoundingClientRect(); }")?;

        let properties = result.preview.expect("JS couldn't give us quad for element").properties;

        let mut prop_map = HashMap::new();

        for prop in properties {
            prop_map.insert(prop.name, prop.value.unwrap().parse::<f64>().unwrap());
        }

        let midpoint = Point {
            x: prop_map["x"] + (prop_map["width"] / 2.0),
            y: prop_map["y"] + (prop_map["height"] / 2.0),
        };

        Ok(midpoint)
    }
}
