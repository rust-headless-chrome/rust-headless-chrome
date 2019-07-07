use std::collections::HashMap;
use std::fmt::Debug;

use failure::{Fail, Fallible};
use log::*;

use crate::browser::tab::point::Point;
use crate::browser::tab::NoElementFound;
use crate::protocol::dom;
use crate::protocol::page;
use crate::protocol::runtime;

mod box_model;

pub use box_model::{BoxModel, ElementQuad};

/// A handle to a [DOM Element](https://developer.mozilla.org/en-US/docs/Web/API/Element).
///
/// Typically you get access to these by passing `Tab.wait_for_element` a CSS selector. Once
/// you have a handle to an element, you can click it, type into it, inspect its
/// attributes, and more. You can even run a JavaScript function inside the tab which can reference
/// the element via `this`.
pub struct Element<'a> {
    pub remote_object_id: String,
    pub backend_node_id: dom::NodeId,
    pub parent: &'a super::Tab,
}

impl<'a> Debug for Element<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Element {}", self.backend_node_id)?;
        Ok(())
    }
}

impl<'a> Element<'a> {
    /// Using a 'node_id', of the type returned by QuerySelector and QuerySelectorAll, this finds
    /// the 'backend_node_id' and 'remote_object_id' which are stable identifiers, unlike node_id.
    /// We use these two when making various calls to the API because of that.
    pub fn new(parent: &'a super::Tab, node_id: dom::NodeId) -> Fallible<Self> {
        if node_id == 0 {
            return Err(NoElementFound {}.into());
        }

        let backend_node_id = parent
            .describe_node(node_id)
            .map_err(NoElementFound::map)?
            .backend_node_id;

        let remote_object_id = {
            let object = parent
                .call_method(dom::methods::ResolveNode {
                    backend_node_id: Some(backend_node_id),
                })?
                .object;
            object.object_id.expect("couldn't find object ID")
        };

        Ok(Element {
            remote_object_id,
            backend_node_id,
            parent,
        })
    }

    /// Moves the mouse to the middle of this element
    pub fn move_mouse_over(&self) -> Fallible<&Self> {
        self.scroll_into_view()?;
        let midpoint = self.get_midpoint()?;
        self.parent.move_mouse_to_point(midpoint)?;
        Ok(self)
    }

    pub fn click(&self) -> Fallible<&Self> {
        self.scroll_into_view()?;
        debug!("Clicking element {:?}", &self);
        let midpoint = self.get_midpoint()?;
        self.parent.click_point(midpoint)?;
        Ok(self)
    }

    pub fn type_into(&self, text: &str) -> Fallible<&Self> {
        self.click()?;

        debug!("Typing into element ( {:?} ): {}", &self, text);

        self.parent.type_str(text)?;

        Ok(self)
    }

    pub fn call_js_fn(
        &self,
        function_declaration: &str,
        await_promise: bool,
    ) -> Fallible<runtime::methods::RemoteObject> {
        let result = self
            .parent
            .call_method(runtime::methods::CallFunctionOn {
                object_id: &self.remote_object_id,
                function_declaration,
                return_by_value: false,
                generate_preview: true,
                silent: false,
                await_promise,
            })?
            .result;

        Ok(result)
    }

    pub fn focus(&self) -> Fallible<&Self> {
        self.scroll_into_view()?;
        self.parent.call_method(dom::methods::Focus {
            backend_node_id: Some(self.backend_node_id),
            ..Default::default()
        })?;
        Ok(self)
    }

    pub fn get_description(&self) -> Fallible<dom::Node> {
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
    /// # use failure::Fallible;
    /// # fn main() -> Fallible<()> {
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
    pub fn capture_screenshot(&self, format: page::ScreenshotFormat) -> Fallible<Vec<u8>> {
        self.scroll_into_view()?;
        self.parent
            .capture_screenshot(format, Some(self.get_box_model()?.content_viewport()), true)
    }

    pub fn set_input_files(&self, file_paths: &[&str]) -> Fallible<&Self> {
        self.parent.call_method(dom::methods::SetFileInputFiles {
            files: file_paths,
            backend_node_id: Some(self.backend_node_id),
            node_id: None,
            object_id: None,
        })?;
        Ok(self)
    }

    /// Scrolls the current element into view
    ///
    /// Used prior to any action applied to the current element to ensure action is duable.
    pub fn scroll_into_view(&self) -> Fallible<&Self> {
        let result = self.call_js_fn(
            "async function() {
                if (!this.isConnected)
                    return 'Node is detached from document';
                if (this.nodeType !== Node.ELEMENT_NODE)
                    return 'Node is not of type HTMLElement';

                const visibleRatio = await new Promise(resolve => {
                    const observer = new IntersectionObserver(entries => {
                        resolve(entries[0].intersectionRatio);
                        observer.disconnect();
                    });
                    observer.observe(this);
                });

                if (visibleRatio !== 1.0)
                    this.scrollIntoView({
                        block: 'center',
                        inline: 'center',
                        behavior: 'instant'
                    });
                return false;
            }",
            true,
        )?;

        if result.object_type == "string" {
            let error_text = result.value.unwrap().as_str().unwrap().to_string();
            return Err(ScrollFailed { error_text }.into());
        }

        Ok(self)
    }

    pub fn get_attributes(&self) -> Fallible<Option<dom::NodeAttributes>> {
        let description = self.get_description()?;
        Ok(description.attributes)
    }

    /// Get boxes for this element
    pub fn get_box_model(&self) -> Fallible<BoxModel> {
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

    pub fn get_midpoint(&self) -> Fallible<Point> {
        let return_object = self.parent.call_method(dom::methods::GetContentQuads {
            node_id: None,
            backend_node_id: Some(self.backend_node_id),
            object_id: None,
        })?;
        let raw_quad = return_object.quads.first().unwrap();
        let input_quad = ElementQuad::from_raw_points(&raw_quad);

        Ok((input_quad.bottom_right + input_quad.top_left) / 2.0)
    }

    pub fn get_js_midpoint(&self) -> Fallible<Point> {
        let result =
            self.call_js_fn("function(){ return this.getBoundingClientRect(); }", false)?;

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

#[derive(Debug, Fail)]
#[fail(display = "Scrolling element into view failed: {}", error_text)]
struct ScrollFailed {
    error_text: String,
}
