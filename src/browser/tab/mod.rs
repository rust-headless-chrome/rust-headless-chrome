use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use failure::{Error, Fail};
use log::*;
use serde;

use element::Element;
use point::Point;

use crate::browser::Transport;
use crate::protocol;
use crate::protocol::page::methods::Navigate;
use crate::protocol::target::TargetId;
use crate::protocol::target::TargetInfo;
use crate::protocol::Event;
use crate::protocol::{dom, input, page, profiler, target};

use super::transport::SessionId;
use super::waiting_helpers::{wait_for, WaitOptions};

pub mod element;
mod keys;
mod point;

/// A handle to a single page. Exposes methods for simulating user actions (clicking,
/// typing), and also for getting information about the DOM and other parts of the page.
pub struct Tab {
    target_id: TargetId,
    transport: Arc<Transport>,
    session_id: SessionId,
    navigating: Arc<AtomicBool>,
    target_info: Arc<Mutex<TargetInfo>>,
}

#[derive(Debug, Fail)]
#[fail(display = "No element found for selector: {}", selector)]
pub struct NoElementFound {
    selector: String,
}

#[derive(Debug, Fail)]
#[fail(display = "Navigate failed: {}", error_text)]
pub struct NavigationFailed {
    error_text: String,
}

impl<'a> Tab {
    pub fn new(target_info: TargetInfo, transport: Arc<Transport>) -> Result<Self, Error> {
        let target_id = target_info.target_id.clone();

        let session_id = transport
            .call_method_on_browser(target::methods::AttachToTarget {
                target_id: &target_id,
                flatten: None,
            })?
            .session_id
            .into();

        debug!("New tab attached with session ID: {:?}", session_id);

        let target_info_mutex = Arc::new(Mutex::new(target_info));

        let tab = Self {
            target_id,
            transport,
            session_id,
            navigating: Arc::new(AtomicBool::new(false)),
            target_info: target_info_mutex,
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

    pub fn get_url(&self) -> String {
        let info = self.target_info.lock().unwrap();
        info.url.clone()
    }

    fn start_event_handler_thread(&self) {
        let incoming_events_rx = self
            .transport
            .listen_to_target_events(self.session_id.clone());
        let navigating = Arc::clone(&self.navigating);

        std::thread::spawn(move || {
            for event in incoming_events_rx {
                trace!("{:?}", &event);
                if let Event::Lifecycle(lifecycle_event) = event {
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
                }
            }
        });
    }

    pub fn call_method<C>(&self, method: C) -> Result<C::ReturnObject, Error>
    where
        C: protocol::Method + serde::Serialize + std::fmt::Debug,
    {
        self.transport
            .call_method_on_target(self.session_id.clone(), method)
    }

    pub fn wait_until_navigated(&self) -> Result<&Self, Error> {
        trace!("waiting to start navigating");
        // wait for navigating to go to true
        let navigating = Arc::clone(&self.navigating);
        wait_for(
            || {
                if navigating.load(Ordering::SeqCst) {
                    Some(true)
                } else {
                    None
                }
            },
            WaitOptions {
                timeout_ms: 15_000,
                sleep_ms: 100,
            },
        )?;
        debug!("A tab started navigating");

        wait_for(
            || {
                if navigating.load(Ordering::SeqCst) {
                    None
                } else {
                    Some(true)
                }
            },
            WaitOptions {
                timeout_ms: 15_000,
                sleep_ms: 100,
            },
        )?;
        debug!("A tab finished navigating");

        Ok(self)
    }

    pub fn navigate_to(&self, url: &str) -> Result<&Self, Error> {
        let return_object = self.call_method(Navigate { url })?;
        if let Some(error_text) = return_object.error_text {
            return Err(NavigationFailed { error_text }.into());
        }

        info!("Navigating a tab to {}", url);

        Ok(self)
    }

    pub fn wait_for_element(&'a self, selector: &'a str) -> Result<Element<'a>, Error> {
        self.wait_for_element_with_custom_timeout(selector, 15_000)
    }

    pub fn wait_for_element_with_custom_timeout(
        &'a self,
        selector: &'a str,
        timeout_ms: u64,
    ) -> Result<Element<'a>, Error> {
        debug!("Waiting for element with selector: {}", selector);
        wait_for(
            || {
                if let Ok(element) = self.find_element(selector) {
                    if element.get_midpoint().is_ok() {
                        Some(element)
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            WaitOptions {
                timeout_ms,
                sleep_ms: 1000,
            },
        )
    }

    pub fn find_element(&'a self, selector: &'a str) -> Result<Element<'a>, Error> {
        trace!("Looking up element via selector: {}", selector);

        let node_id = {
            let root_node_id = self
                .call_method(dom::methods::GetDocument {
                    depth: Some(0),
                    pierce: Some(false),
                })?
                .root
                .node_id;

            self.call_method(dom::methods::QuerySelector {
                node_id: root_node_id,
                selector,
            })?
            .node_id
        };

        if node_id == 0 {
            return Err(NoElementFound {
                selector: selector.to_string(),
            }
            .into());
        }

        let backend_node_id = self.describe_node(node_id)?.backend_node_id;

        let remote_object_id = {
            let object = self
                .call_method(dom::methods::ResolveNode {
                    backend_node_id: Some(backend_node_id),
                })?
                .object;
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
        let node = self
            .call_method(dom::methods::DescribeNode {
                node_id: Some(node_id),
                backend_node_id: None,
                depth: Some(100),
            })?
            .node;
        Ok(node)
    }

    pub fn type_str(&self, string_to_type: &str) -> Result<&Self, Error> {
        for c in string_to_type.split("") {
            // split call above will have empty string at start and end which we won't type
            if c == "" {
                continue;
            }
            self.press_key(c)?;
        }
        Ok(self)
    }

    pub fn press_key(&self, key: &str) -> Result<&Self, Error> {
        let definition = keys::get_key_definition(key)?;

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
        Ok(self)
    }

    pub fn click_point(&self, point: Point) -> Result<&Self, Error> {
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
        self.call_method(input::methods::DispatchMouseEvent {
            event_type: "mousePressed",
            x: point.x,
            y: point.y,
            button: Some("left"),
            click_count: Some(1),
        })?;
        self.call_method(input::methods::DispatchMouseEvent {
            event_type: "mouseReleased",
            x: point.x,
            y: point.y,
            button: Some("left"),
            click_count: Some(1),
        })?;
        Ok(self)
    }

    /// Capture a screenshot of the current page.
    ///
    /// If `clip` is given, the screenshot is taken of the specified region only.
    /// `Element::get_box_model()` can be used to get regions of certains elements
    /// on the page; there is also `Element::capture_screenhot()` as a shorthand.
    ///
    /// If `from_surface` is true, the screenshot is taken from the surface rather than
    /// the view.
    ///
    /// ```rust,no_run
    /// # use failure::Error;
    /// # fn main() -> Result<(), Error> {
    /// #
    /// use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder};
    /// let browser = Browser::new(LaunchOptionsBuilder::default().build().unwrap())?;
    /// let tab = browser.wait_for_initial_tab()?;
    /// let viewport = tab.navigate_to("https://en.wikipedia.org/wiki/WebKit")?
    ///     .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
    ///     .get_box_model()?
    ///     .margin_viewport();
    ///  let png_data = tab.capture_screenshot(ScreenshotFormat::PNG, Some(viewport), true)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn capture_screenshot(
        &self,
        format: page::ScreenshotFormat,
        clip: Option<page::Viewport>,
        from_surface: bool,
    ) -> Result<Vec<u8>, Error> {
        let (format, quality) = match format {
            page::ScreenshotFormat::JPEG(quality) => {
                (page::InternalScreenshotFormat::JPEG, quality)
            }
            page::ScreenshotFormat::PNG => (page::InternalScreenshotFormat::PNG, None),
        };
        let data = self
            .call_method(page::methods::CaptureScreenshot {
                format,
                clip,
                quality,
                from_surface,
            })?
            .data;
        base64::decode(&data).map_err(Into::into)
    }

    /// Reloads given page optionally ignoring the cache
    ///
    /// If `ignore_cache` is true, the browser cache is ignored (as if the user pressed Shift+F5).
    /// If `script_to_evaluate` is given, the script will be injected into all frames of the
    /// inspected page after reload. Argument will be ignored if reloading dataURL origin.
    pub fn reload(
        &self,
        ignore_cache: bool,
        script_to_evaluate: Option<&str>,
    ) -> Result<&Self, Error> {
        self.call_method(page::methods::Reload {
            ignore_cache,
            script_to_evaluate,
        })?;
        Ok(self)
    }

    /// Enables the profiler
    pub fn enable_profiler(&self) -> Result<&Self, Error> {
        self.call_method(profiler::methods::Enable {})?;

        Ok(self)
    }

    /// Disables the profiler
    pub fn disable_profiler(&self) -> Result<&Self, Error> {
        self.call_method(profiler::methods::Disable {})?;

        Ok(self)
    }

    /// Starts tracking which lines of JS have been executed
    ///
    /// Will return error unless `enable_profiler` has been called.
    ///
    /// Equivalent to hitting the record button in the "coverage" tab in Chrome DevTools.
    /// See the file `tests/coverage.rs` for an example.
    ///
    /// By default we enable the 'detailed' flag on StartPreciseCoverage, which enables block-level
    /// granularity, and also enable 'call_count' (which when disabled always sets count to 1 or 0).
    ///
    pub fn start_js_coverage(&self) -> Result<&Self, Error> {
        self.call_method(profiler::methods::StartPreciseCoverage {
            call_count: Some(true),
            detailed: Some(true),
        })?;
        Ok(self)
    }

    /// Stops tracking which lines of JS have been executed
    /// If you're finished with the profiler, don't forget to call `disable_profiler`.
    pub fn stop_js_coverage(&self) -> Result<&Self, Error> {
        self.call_method(profiler::methods::StopPreciseCoverage {})?;
        Ok(self)
    }

    /// Collect coverage data for the current isolate, and resets execution counters.
    ///
    /// Precise code coverage needs to have started (see `start_js_coverage`).
    ///
    /// Will only send information about code that's been executed since this method was last
    /// called, or (if this is the first time) since calling `start_js_coverage`.
    /// Another way of thinking about it is: every time you call this, the call counts for
    /// FunctionRanges are reset after returning.
    ///
    /// The format of the data is a little unintuitive, see here for details:
    /// https://chromedevtools.github.io/devtools-protocol/tot/Profiler#type-ScriptCoverage
    pub fn take_precise_js_coverage(&self) -> Result<Vec<profiler::ScriptCoverage>, Error> {
        let script_coverages = self
            .call_method(profiler::methods::TakePreciseCoverage {})?
            .result;
        Ok(script_coverages)
    }
}
