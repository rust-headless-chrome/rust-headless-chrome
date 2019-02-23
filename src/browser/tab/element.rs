use failure::Error;
use log::*;

use super::point::Point;
use crate::protocol::dom;
use crate::protocol::page;
use crate::protocol::runtime;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub struct ElementQuad {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}

impl ElementQuad {
    pub fn from_raw_points(raw_quad: &[f64; 8]) -> Self {
        Self {
            top_left: Point {
                x: raw_quad[0],
                y: raw_quad[1],
            },
            top_right: Point {
                x: raw_quad[2],
                y: raw_quad[3],
            },
            bottom_right: Point {
                x: raw_quad[4],
                y: raw_quad[5],
            },
            bottom_left: Point {
                x: raw_quad[6],
                y: raw_quad[7],
            },
        }
    }

    pub fn height(&self) -> f64 {
        self.bottom_left.y - self.top_left.y
    }

    pub fn width(&self) -> f64 {
        self.top_right.x - self.top_left.x
    }

    /// The width divided by the height
    pub fn aspect_ratio(&self) -> f64 {
        self.width() / self.height()
    }
}

#[derive(Debug, Clone)]
pub struct BoxModel {
    pub content: ElementQuad,
    pub padding: ElementQuad,
    pub border: ElementQuad,
    pub margin: ElementQuad,
    pub width: u64,
    pub height: u64,
}

impl BoxModel {
    /// Create a `page::Viewport` equal to the content-box, using a scale of 1.0
    pub fn content_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.content.top_left.x,
            y: self.content.top_left.y,
            width: self.content.width(),
            height: self.content.height(),
            scale: 1.0,
        }
    }

    /// Create a `page::Viewport` equal to the padding-box, using a scale of 1.0
    pub fn padding_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.padding.top_left.x,
            y: self.padding.top_left.y,
            width: self.padding.width(),
            height: self.padding.height(),
            scale: 1.0,
        }
    }

    /// Create a `page::Viewport` equal to the border-box, using a scale of 1.0
    pub fn border_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.border.top_left.x,
            y: self.border.top_left.y,
            width: self.border.width(),
            height: self.border.height(),
            scale: 1.0,
        }
    }

    /// Create a `page::Viewport` equal to the margin-box, using a scale of 1.0
    pub fn margin_viewport(&self) -> page::Viewport {
        page::Viewport {
            x: self.margin.top_left.x,
            y: self.margin.top_left.y,
            width: self.margin.width(),
            height: self.margin.height(),
            scale: 1.0,
        }
    }
}

pub struct Element<'a> {
    pub remote_object_id: String,
    pub backend_node_id: dom::NodeId,
    pub parent: &'a super::Tab,
    pub found_via_selector: &'a str,
}

impl<'a> Element<'a> {
    pub fn click(&self) -> Result<&Self, Error> {
        debug!("Clicking element found via {}", self.found_via_selector);

        let midpoint = self.get_midpoint()?;
        self.parent.click_point(midpoint)?;
        Ok(self)
    }

    pub fn type_into(&self, text: &str) -> Result<&Self, Error> {
        self.click()?;

        debug!(
            "Typing into element ( {} ): {}",
            self.found_via_selector, text
        );

        self.parent.type_str(text)?;

        Ok(self)
    }

    pub fn call_js_fn(
        &self,
        function_declaration: &str,
    ) -> Result<runtime::methods::RemoteObject, Error> {
        let result = self
            .parent
            .call_method(runtime::methods::CallFunctionOn {
                object_id: &self.remote_object_id,
                function_declaration,
                return_by_value: false,
                generate_preview: true,
                silent: false,
            })?
            .result;

        Ok(result)
    }

    pub fn focus(&self) -> Result<&Self, Error> {
        self.parent.call_method(dom::methods::Focus {
            backend_node_id: Some(self.backend_node_id),
            ..Default::default()
        })?;
        Ok(self)
    }

    pub fn get_description(&self) -> Result<dom::Node, Error> {
        let node = self
            .parent
            .call_method(dom::methods::DescribeNode {
                node_id: None,
                backend_node_id: Some(self.backend_node_id),
                depth: Some(100),
            })?
            .node;
        Ok(node)
    }

    /// Capture a screenshot of this element.
    ///
    /// The screenshot is taken from the surface using this element's content-box.
    ///
    /// ```rust,no_run
    /// # use failure::Error;
    /// # fn main() -> Result<(), Error> {
    /// #
    /// use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
    /// let browser = Browser::new(LaunchOptionsBuilder::default().build().unwrap())?;
    /// let png_data = browser.wait_for_initial_tab()?
    ///     .navigate_to("https://en.wikipedia.org/wiki/WebKit")?
    ///     .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
    ///     .capture_screenshot(ScreenshotFormat::PNG)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn capture_screenshot(&self, format: page::ScreenshotFormat) -> Result<Vec<u8>, Error> {
        self.parent
            .capture_screenshot(format, Some(self.get_box_model()?.content_viewport()), true)
    }

    pub fn set_input_files(&self, file_paths: &[&str]) -> Result<&Self, Error> {
        self.parent.call_method(dom::methods::SetFileInputFiles {
            files: file_paths,
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_id: None,
        })?;
        Ok(self)
    }

    pub fn get_attributes(&self) -> Result<Option<dom::NodeAttributes>, Error> {
        let description = self.get_description()?;
        Ok(description.attributes)
    }

    /// Get boxes for this element
    pub fn get_box_model(&self) -> Result<BoxModel, Error> {
        let model = self
            .parent
            .call_method(dom::methods::GetBoxModel {
                node_id: None,
                backend_node_id: Some(self.backend_node_id),
                object_id: None,
            })?
            .model;
        Ok(BoxModel {
            content: ElementQuad::from_raw_points(&model.content),
            padding: ElementQuad::from_raw_points(&model.padding),
            border: ElementQuad::from_raw_points(&model.border),
            margin: ElementQuad::from_raw_points(&model.margin),
            width: model.width,
            height: model.height,
        })
    }

    pub fn get_midpoint(&self) -> Result<Point, Error> {
        let return_object = self.parent.call_method(dom::methods::GetContentQuads {
            node_id: None,
            backend_node_id: Some(self.backend_node_id),
            object_id: None,
        })?;
        let raw_quad = return_object.quads.first().unwrap();
        let input_quad = ElementQuad::from_raw_points(&raw_quad);

        Ok((input_quad.bottom_right + input_quad.top_left) / 2.0)
    }

    pub fn get_js_midpoint(&self) -> Result<Point, Error> {
        let result = self.call_js_fn("function(){ return this.getBoundingClientRect(); }")?;

        let properties = result
            .preview
            .expect("JS couldn't give us quad for element")
            .properties;

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
