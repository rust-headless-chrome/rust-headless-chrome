use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock, Weak};
use std::thread;
use std::time::Duration;

use failure::{Error, Fail, Fallible};
use log::*;
use serde;

use element::Element;
use point::Point;

use crate::browser::Transport;
use crate::protocol::dom::{Node, NodeId};
use crate::protocol::page::methods::{
    FileChooserAction, HandleFileChooser, Navigate, SetInterceptFileChooserDialog,
};
use crate::protocol::target::{TargetId, TargetInfo};
use crate::protocol::{
    dom, input, logs, network, page, profiler, runtime, target, Event, RemoteError,
};
use crate::{protocol, protocol::logs::methods::ViolationSetting, util};

use super::transport::SessionId;
use crate::protocol::network::Cookie;
use std::thread::sleep;

pub mod element;
mod keys;
mod point;

#[derive(Debug)]
pub enum RequestInterceptionDecision {
    Continue,
    // TODO: Error
    Response(String),
}

pub type RequestInterceptor = Box<
    dyn Fn(
            Arc<Transport>,
            SessionId,
            protocol::network::events::RequestInterceptedEventParams,
        ) -> RequestInterceptionDecision
        + Send
        + Sync,
>;

#[rustfmt::skip]
pub type ResponseHandler = Box<
    dyn Fn(
        protocol::network::events::ResponseReceivedEventParams,
        &dyn Fn() -> Result<
            protocol::network::methods::GetResponseBodyReturnObject,
            failure::Error,
        >,
    ) + Send
    + Sync,
>;

pub trait EventListener<T> {
    fn on_event(&self, event: &T) -> ();
}

impl<T, F: Fn(&T) + Send + Sync> EventListener<T> for F {
    fn on_event(&self, event: &T) {
        self(&event);
    }
}

type SyncSendEvent = dyn EventListener<Event> + Send + Sync;

/// A handle to a single page. Exposes methods for simulating user actions (clicking,
/// typing), and also for getting information about the DOM and other parts of the page.
pub struct Tab {
    target_id: TargetId,
    transport: Arc<Transport>,
    session_id: SessionId,
    navigating: Arc<AtomicBool>,
    target_info: Arc<Mutex<TargetInfo>>,
    request_interceptor: Arc<Mutex<RequestInterceptor>>,
    response_handler: Arc<Mutex<Option<ResponseHandler>>>,
    default_timeout: Arc<RwLock<Duration>>,
    event_listeners: Arc<Mutex<Vec<Arc<SyncSendEvent>>>>,
    slow_motion_multiplier: Arc<RwLock<f64>>, // there's no AtomicF64, otherwise would use that
}

#[derive(Debug, Fail)]
#[fail(display = "No element found")]
pub struct NoElementFound {}

#[derive(Debug, Fail)]
#[fail(display = "Navigate failed: {}", error_text)]
pub struct NavigationFailed {
    error_text: String,
}

impl NoElementFound {
    pub fn map(error: Error) -> Error {
        match error.downcast::<RemoteError>() {
            Ok(remote_error) => {
                match remote_error.message.as_ref() {
                    // This error is expected and occurs while the page is still loading,
                    // hence we shadow it and respond the element is not found
                    "Could not find node with given id" => Self {}.into(),

                    // Any other error is unexpected and should be reported
                    _ => remote_error.into(),
                }
            }
            // Return original error if downcasting to RemoteError fails
            Err(original_error) => original_error,
        }
    }
}

impl<'a> Tab {
    pub fn new(target_info: TargetInfo, transport: Arc<Transport>) -> Fallible<Self> {
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
            request_interceptor: Arc::new(Mutex::new(Box::new(
                |_transport, _session_id, _interception| RequestInterceptionDecision::Continue,
            ))),
            response_handler: Arc::new(Mutex::new(None)),
            default_timeout: Arc::new(RwLock::new(Duration::from_secs(3))),
            event_listeners: Arc::new(Mutex::new(Vec::new())),
            slow_motion_multiplier: Arc::new(RwLock::new(0.0)),
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

    /// Fetches the most recent info about this target
    pub fn get_target_info(&self) -> Fallible<TargetInfo> {
        Ok(self
            .call_method(target::methods::GetTargetInfo {
                target_id: self.get_target_id(),
            })?
            .target_info)
    }

    pub fn get_browser_context_id(&self) -> Fallible<Option<String>> {
        Ok(self.get_target_info()?.browser_context_id)
    }

    pub fn get_url(&self) -> String {
        let info = self.target_info.lock().unwrap();
        info.url.clone()
    }

    /// Allows overriding user agent with the given string.
    pub fn set_user_agent(
        &self,
        user_agent: &str,
        accept_language: Option<&str>,
        platform: Option<&str>,
    ) -> Fallible<()> {
        self.call_method(network::methods::SetUserAgentOverride {
            user_agent,
            accept_language,
            platform,
        })
        .map(|_| ())
    }

    fn start_event_handler_thread(&self) {
        let transport: Arc<Transport> = Arc::clone(&self.transport);
        let incoming_events_rx = self
            .transport
            .listen_to_target_events(self.session_id.clone());
        let navigating = Arc::clone(&self.navigating);
        let interceptor_mutex = Arc::clone(&self.request_interceptor);
        let response_handler_mutex = self.response_handler.clone();
        let session_id = self.session_id.clone();
        let listeners_mutex = Arc::clone(&self.event_listeners);

        thread::spawn(move || {
            for event in incoming_events_rx {
                let listeners = listeners_mutex.lock().unwrap();
                listeners.iter().for_each(|listener| {
                    listener.on_event(&event);
                });

                match event {
                    Event::Lifecycle(lifecycle_event) => {
                        let event_name = lifecycle_event.params.name.as_ref();
                        trace!("Lifecycle event: {}", event_name);
                        match event_name {
                            "networkAlmostIdle" => {
                                navigating.store(false, Ordering::SeqCst);
                            }
                            "init" => {
                                navigating.store(true, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    }
                    Event::RequestIntercepted(interception_event) => {
                        let id = interception_event.params.interception_id.clone();
                        let interceptor = interceptor_mutex.lock().unwrap();
                        let decision = interceptor(
                            Arc::clone(&transport),
                            session_id.clone(),
                            interception_event.params,
                        );
                        match decision {
                            RequestInterceptionDecision::Continue => {
                                let method = network::methods::ContinueInterceptedRequest {
                                    interception_id: &id,
                                    ..Default::default()
                                };
                                let result =
                                    transport.call_method_on_target(session_id.clone(), method);
                                if result.is_err() {
                                    warn!("Tried to continue request after connection was closed");
                                }
                            }
                            RequestInterceptionDecision::Response(response_str) => {
                                let method = network::methods::ContinueInterceptedRequest {
                                    interception_id: &id,
                                    raw_response: Some(&response_str),
                                    ..Default::default()
                                };
                                let result =
                                    transport.call_method_on_target(session_id.clone(), method);
                                if result.is_err() {
                                    warn!("Tried to continue request after connection was closed");
                                }
                            }
                        }
                    }
                    Event::ResponseReceived(ev) => {
                        if let Some(handler) = response_handler_mutex.lock().unwrap().as_ref() {
                            let request_id = ev.params.request_id.clone();
                            let retrieve_body = || {
                                let method = network::methods::GetResponseBody {
                                    request_id: &request_id,
                                };
                                transport.call_method_on_target(session_id.clone(), method)
                            };
                            handler(ev.params, &retrieve_body);
                        }
                    }
                    _ => {
                        let mut raw_event = format!("{:?}", event);
                        raw_event.truncate(50);
                        trace!("Unhandled event: {}", raw_event);
                    }
                }
            }
            info!("finished tab's event handling loop");
        });
    }

    pub fn call_method<C>(&self, method: C) -> Fallible<C::ReturnObject>
    where
        C: protocol::Method + serde::Serialize + std::fmt::Debug,
    {
        trace!("Calling method: {:?}", method);
        let result = self
            .transport
            .call_method_on_target(self.session_id.clone(), method);
        let mut result_string = format!("{:?}", result);
        result_string.truncate(70);
        trace!("Got result: {:?}", result_string);
        result
    }

    pub fn wait_until_navigated(&self) -> Fallible<&Self> {
        let navigating = Arc::clone(&self.navigating);

        util::Wait::with_timeout(Duration::from_secs(20)).until(|| {
            if navigating.load(Ordering::SeqCst) {
                None
            } else {
                Some(true)
            }
        })?;
        debug!("A tab finished navigating");

        Ok(self)
    }

    pub fn navigate_to(&self, url: &str) -> Fallible<&Self> {
        let return_object = self.call_method(Navigate { url })?;
        if let Some(error_text) = return_object.error_text {
            return Err(NavigationFailed { error_text }.into());
        }

        let navigating = Arc::clone(&self.navigating);
        navigating.store(true, Ordering::SeqCst);

        info!("Navigating a tab to {}", url);

        Ok(self)
    }

    /// Set default timeout for the tab
    ///
    /// This will be applied to all [wait_for_element](Tab::wait_for_element) and [wait_for_elements](Tab::wait_for_elements) calls for this tab
    ///
    /// ```rust
    /// # use failure::Fallible;
    /// # fn main() -> Fallible<()> {
    /// # use headless_chrome::Browser;
    /// # let browser = Browser::default()?;
    /// let tab = browser.wait_for_initial_tab()?;
    /// tab.set_default_timeout(std::time::Duration::from_secs(5));
    /// #
    /// # Ok(())
    /// # }

    /// ```
    pub fn set_default_timeout(&self, timeout: Duration) -> &Self {
        let mut current_timeout = self.default_timeout.write().unwrap();
        *current_timeout = timeout;
        &self
    }

    /// Analogous to Puppeteer's ['slowMo' option](https://github.com/GoogleChrome/puppeteer/blob/v1.20.0/docs/api.md#puppeteerconnectoptions),
    /// but with some differences:
    ///
    /// * It doesn't add a delay after literally every message sent via the protocol, but instead
    ///   just for:
    ///     * clicking a specific point on the page (default: 100ms before moving the mouse, 250ms
    ///       before pressing and releasting mouse button)
    ///     * pressing a key (default: 25 ms)
    ///     * reloading the page (default: 100ms)
    ///     * closing a tab (default: 100ms)
    /// * Instead of an absolute number of milliseconds, it's a multiplier, so that we can delay
    ///   longer on certain actions like clicking or moving the mouse, and shorter on others like
    ///   on pressing a key (or the individual 'mouseDown' and 'mouseUp' actions that go across the
    ///   wire. If the delay was always the same, filling out a form (e.g.) would take ages).
    ///
    /// By default the multiplier is set to zero, which effectively disables the slow motion.
    ///
    /// The defaults for the various actions (i.e. how long we sleep for when
    /// multiplier is 1.0) are supposed to be just slow enough to help a human see what's going on
    /// as a test runs.
    pub fn set_slow_motion_multiplier(&self, multiplier: f64) -> &Self {
        let mut slow_motion_multiplier = self.slow_motion_multiplier.write().unwrap();
        *slow_motion_multiplier = multiplier;
        &self
    }

    fn optional_slow_motion_sleep(&self, millis: u64) {
        let multiplier = self.slow_motion_multiplier.read().unwrap();
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let scaled_millis = millis * *multiplier as u64;
        sleep(Duration::from_millis(scaled_millis));
    }

    pub fn wait_for_element(&self, selector: &str) -> Fallible<Element<'_>> {
        self.wait_for_element_with_custom_timeout(selector, *self.default_timeout.read().unwrap())
    }

    pub fn wait_for_element_with_custom_timeout(
        &self,
        selector: &str,
        timeout: std::time::Duration,
    ) -> Fallible<Element<'_>> {
        debug!("Waiting for element with selector: {}", selector);
        util::Wait::with_timeout(timeout).strict_until(
            || self.find_element(selector),
            Error::downcast::<NoElementFound>,
        )
    }

    pub fn wait_for_elements(&self, selector: &str) -> Fallible<Vec<Element<'_>>> {
        debug!("Waiting for element with selector: {}", selector);
        util::Wait::with_timeout(*self.default_timeout.read().unwrap()).strict_until(
            || self.find_elements(selector),
            Error::downcast::<NoElementFound>,
        )
    }

    /// Returns the first element in the document which matches the given CSS selector.
    ///
    /// Equivalent to the following JS:
    ///
    /// ```js
    /// document.querySelector(selector)
    /// ```
    ///
    /// ```rust
    /// # use failure::Fallible;
    /// # // Awful hack to get access to testing utils common between integration, doctest, and unit tests
    /// # mod server {
    /// #     include!("../../testing_utils/server.rs");
    /// # }
    /// # fn main() -> Fallible<()> {
    /// #
    /// use headless_chrome::Browser;
    ///
    /// let browser = Browser::default()?;
    /// let initial_tab = browser.wait_for_initial_tab()?;
    ///
    /// let file_server = server::Server::with_dumb_html(include_str!("../../../tests/simple.html"));
    /// let element = initial_tab.navigate_to(&file_server.url())?
    ///     .wait_until_navigated()?
    ///     .find_element("div#foobar")?;
    /// let attrs = element.get_attributes()?.unwrap();
    /// assert_eq!(attrs["id"], "foobar");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn find_element(&self, selector: &str) -> Fallible<Element<'_>> {
        trace!("Looking up element via selector: {}", selector);

        let root_node_id = self.get_document()?.node_id;
        self.run_query_selector_on_node(root_node_id, selector)
    }

    pub fn run_query_selector_on_node(
        &self,
        node_id: NodeId,
        selector: &str,
    ) -> Fallible<Element<'_>> {
        let node_id = self
            .call_method(dom::methods::QuerySelector { node_id, selector })
            .map_err(NoElementFound::map)?
            .node_id;

        Element::new(&self, node_id)
    }

    pub fn run_query_selector_all_on_node(
        &self,
        node_id: NodeId,
        selector: &str,
    ) -> Fallible<Vec<Element<'_>>> {
        let node_ids = self
            .call_method(dom::methods::QuerySelectorAll { node_id, selector })
            .map_err(NoElementFound::map)?
            .node_ids;

        node_ids
            .iter()
            .map(|node_id| Element::new(&self, *node_id))
            .collect()
    }

    pub fn get_document(&self) -> Fallible<Node> {
        Ok(self
            .call_method(dom::methods::GetDocument {
                depth: Some(0),
                pierce: Some(false),
            })?
            .root)
    }

    pub fn find_elements(&self, selector: &str) -> Fallible<Vec<Element<'_>>> {
        trace!("Looking up elements via selector: {}", selector);

        let root_node_id = self.get_document()?.node_id;
        let node_ids = self
            .call_method(dom::methods::QuerySelectorAll {
                node_id: root_node_id,
                selector,
            })
            .map_err(NoElementFound::map)?
            .node_ids;

        if node_ids.is_empty() {
            return Err(NoElementFound {}.into());
        }

        node_ids
            .into_iter()
            .map(|node_id| Element::new(&self, node_id))
            .collect()
    }

    pub fn describe_node(&self, node_id: dom::NodeId) -> Fallible<dom::Node> {
        let node = self
            .call_method(dom::methods::DescribeNode {
                node_id: Some(node_id),
                backend_node_id: None,
                depth: Some(100),
            })?
            .node;
        Ok(node)
    }

    pub fn type_str(&self, string_to_type: &str) -> Fallible<&Self> {
        for c in string_to_type.split("") {
            // split call above will have empty string at start and end which we won't type
            if c == "" {
                continue;
            }
            self.press_key(c)?;
        }
        Ok(self)
    }

    pub fn press_key(&self, key: &str) -> Fallible<&Self> {
        let definition = keys::get_key_definition(key)?;

        // See https://github.com/GoogleChrome/puppeteer/blob/62da2366c65b335751896afbb0206f23c61436f1/lib/Input.js#L114-L115
        let text = definition.text.or_else(|| {
            if definition.key.len() == 1 {
                Some(definition.key)
            } else {
                None
            }
        });

        // See https://github.com/GoogleChrome/puppeteer/blob/62da2366c65b335751896afbb0206f23c61436f1/lib/Input.js#L52
        let key_down_event_type = if text.is_some() {
            "keyDown"
        } else {
            "rawKeyDown"
        };

        let key = Some(definition.key);
        let code = Some(definition.code);

        self.optional_slow_motion_sleep(25);

        self.call_method(input::methods::DispatchKeyEvent {
            event_type: key_down_event_type,
            key,
            text,
            code: Some(definition.code),
            windows_virtual_key_code: definition.key_code,
            native_virtual_key_code: definition.key_code,
        })?;
        self.call_method(input::methods::DispatchKeyEvent {
            event_type: "keyUp",
            key,
            text,
            code,
            windows_virtual_key_code: definition.key_code,
            native_virtual_key_code: definition.key_code,
        })?;
        Ok(self)
    }

    /// Moves the mouse to this point (dispatches a mouseMoved event)
    pub fn move_mouse_to_point(&self, point: Point) -> Fallible<&Self> {
        if point.x == 0.0 && point.y == 0.0 {
            warn!("Midpoint of element shouldn't be 0,0. Something is probably wrong.")
        }

        self.optional_slow_motion_sleep(100);

        self.call_method(input::methods::DispatchMouseEvent {
            event_type: "mouseMoved",
            x: point.x,
            y: point.y,
            ..Default::default()
        })?;

        Ok(self)
    }

    pub fn click_point(&self, point: Point) -> Fallible<&Self> {
        trace!("Clicking point: {:?}", point);
        if point.x == 0.0 && point.y == 0.0 {
            warn!("Midpoint of element shouldn't be 0,0. Something is probably wrong.")
        }

        self.move_mouse_to_point(point)?;

        self.optional_slow_motion_sleep(250);
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
    /// # use failure::Fallible;
    /// # fn main() -> Fallible<()> {
    /// #
    /// use headless_chrome::{protocol::page::ScreenshotFormat, Browser, LaunchOptions};
    /// let browser = Browser::new(LaunchOptions::default_builder().build().unwrap())?;
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
    ) -> Fallible<Vec<u8>> {
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

    pub fn print_to_pdf(&self, options: Option<page::PrintToPdfOptions>) -> Fallible<Vec<u8>> {
        let data = self
            .call_method(page::methods::PrintToPdf { options })?
            .data;
        base64::decode(&data).map_err(Into::into)
    }

    /// Reloads given page optionally ignoring the cache
    ///
    /// If `ignore_cache` is true, the browser cache is ignored (as if the user pressed Shift+F5).
    /// If `script_to_evaluate` is given, the script will be injected into all frames of the
    /// inspected page after reload. Argument will be ignored if reloading dataURL origin.
    pub fn reload(&self, ignore_cache: bool, script_to_evaluate: Option<&str>) -> Fallible<&Self> {
        self.optional_slow_motion_sleep(100);
        self.call_method(page::methods::Reload {
            ignore_cache,
            script_to_evaluate,
        })?;
        Ok(self)
    }

    /// Enables the profiler
    pub fn enable_profiler(&self) -> Fallible<&Self> {
        self.call_method(profiler::methods::Enable {})?;

        Ok(self)
    }

    /// Disables the profiler
    pub fn disable_profiler(&self) -> Fallible<&Self> {
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
    pub fn start_js_coverage(&self) -> Fallible<&Self> {
        self.call_method(profiler::methods::StartPreciseCoverage {
            call_count: Some(true),
            detailed: Some(true),
        })?;
        Ok(self)
    }

    /// Stops tracking which lines of JS have been executed
    /// If you're finished with the profiler, don't forget to call `disable_profiler`.
    pub fn stop_js_coverage(&self) -> Fallible<&Self> {
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
    pub fn take_precise_js_coverage(&self) -> Fallible<Vec<profiler::ScriptCoverage>> {
        let script_coverages = self
            .call_method(profiler::methods::TakePreciseCoverage {})?
            .result;
        Ok(script_coverages)
    }

    /// Allows you to inspect outgoing network requests from the tab, and optionally return
    /// your own responses to them
    ///
    /// The `interceptor` argument is a closure which takes this tab's `Transport` and its SessionID
    /// so that you can call methods from within the closure using `transport.call_method_on_target`.
    ///
    /// The closure needs to return a variant of `RequestInterceptionDecision` (so, `Continue` or
    /// `Response(String)`).
    pub fn enable_request_interception(
        &self,
        patterns: &[network::methods::RequestPattern],
        interceptor: RequestInterceptor,
    ) -> Fallible<()> {
        let mut current_interceptor = self.request_interceptor.lock().unwrap();
        *current_interceptor = interceptor;
        self.call_method(network::methods::SetRequestInterception {
            patterns: &patterns,
        })?;
        Ok(())
    }

    /// Once you have an intercepted request, you can choose to let it continue by calling this.
    ///
    /// If you specify a 'modified_response', that's what the requester in the page will receive
    /// instead of what they normally would've received.
    pub fn continue_intercepted_request(
        &self,
        interception_id: &str,
        modified_response: Option<&str>,
    ) -> Fallible<()> {
        self.call_method(network::methods::ContinueInterceptedRequest {
            interception_id,
            error_reason: None,
            raw_response: modified_response,
            url: None,
            method: None,
            post_data: None,
            headers: None,
            auth_challenge_response: None,
        })?;
        Ok(())
    }

    /// Lets you listen for responses, and gives you a way to get the response body too.
    ///
    /// Please note that the 'response' does not include the *body* of the response -- Chrome tells
    /// us about them seperately (because you might quickly get the status code and headers from a
    /// server well before you receive the entire response body which could, after all, be gigabytes
    /// long).
    ///
    /// Currently we leave it up to the caller to decide when to call `fetch_body` (the second
    /// argument to the response handler), although ideally it wouldn't be possible until Chrome has
    /// sent the `Network.loadingFinished` event.
    ///
    /// Currently you can only have one handler registered, but ideally there would be no limit and
    /// we'd give you a mechanism to deregister the handler too.
    pub fn enable_response_handling(&self, handler: ResponseHandler) -> Fallible<()> {
        self.call_method(network::methods::Enable {})?;
        *(self.response_handler.lock().unwrap()) = Some(handler);
        Ok(())
    }

    /// Enables runtime domain.
    pub fn enable_runtime(&self) -> Fallible<&Self> {
        self.call_method(runtime::methods::Enable {})?;

        Ok(self)
    }

    /// Disables runtime domain
    pub fn disable_runtime(&self) -> Fallible<&Self> {
        self.call_method(runtime::methods::Disable {})?;

        Ok(self)
    }

    /// Enables Debugger
    pub fn enable_debugger(&self) -> Fallible<()> {
        self.call_method(protocol::debugger::methods::Enable {})?;
        Ok(())
    }

    /// Disables Debugger
    pub fn disable_debugger(&self) -> Fallible<()> {
        self.call_method(protocol::debugger::methods::Disable {})?;
        Ok(())
    }

    /// Returns source for the script with given id.
    ///
    /// Debugger must be enabled.
    pub fn get_script_source(&self, script_id: &str) -> Fallible<String> {
        Ok(self
            .call_method(protocol::debugger::methods::GetScriptSource { script_id })?
            .script_source)
    }

    /// Enables log domain.
    ///
    /// Sends the entries collected so far to the client by means of the entryAdded notification.
    ///
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Log#method-enable
    pub fn enable_log(&self) -> Fallible<&Self> {
        self.call_method(logs::methods::Enable {})?;

        Ok(self)
    }

    /// Disables log domain
    ///
    /// Prevents further log entries from being reported to the client
    ///
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Log#method-disable
    pub fn disable_log(&self) -> Fallible<&Self> {
        self.call_method(logs::methods::Disable {})?;

        Ok(self)
    }

    /// Starts violation reporting
    ///
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Log#method-startViolationsReport
    pub fn start_violations_report(&self, config: Vec<ViolationSetting>) -> Fallible<&Self> {
        self.call_method(logs::methods::StartViolationsReport { config })?;
        Ok(self)
    }

    /// Stop violation reporting
    ///
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Log#method-stopViolationsReport
    pub fn stop_violations_report(&self) -> Fallible<&Self> {
        self.call_method(logs::methods::StopViolationsReport {})?;
        Ok(self)
    }

    /// Evaluates expression on global object.
    pub fn evaluate(
        &self,
        expression: &str,
        await_promise: bool,
    ) -> Fallible<protocol::runtime::methods::RemoteObject> {
        let result = self
            .call_method(protocol::runtime::methods::Evaluate {
                expression,
                return_by_value: false,
                generate_preview: true,
                silent: false,
                await_promise,
                include_command_line_api: false,
                user_gesture: false,
            })?
            .result;
        Ok(result)
    }

    /// Adds event listener to Event
    ///
    /// Make sure you are enabled domain you are listening events to.
    ///
    /// ## Usage example
    ///
    /// ```rust
    /// # use failure::Fallible;
    /// # use std::sync::Arc;
    /// # fn main() -> Fallible<()> {
    /// #
    /// # use headless_chrome::Browser;
    /// # use headless_chrome::protocol::Event;
    /// # let browser = Browser::default()?;
    /// # let tab = browser.wait_for_initial_tab()?;
    /// tab.enable_log()?;
    /// tab.add_event_listener(Arc::new(move |event: &Event| {
    ///     match event {
    ///         Event::LogEntryAdded(_) => {
    ///             // process event here
    ///         }
    ///         _ => {}
    ///       }
    ///     }))?;
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    ///
    pub fn add_event_listener(
        &self,
        listener: Arc<SyncSendEvent>,
    ) -> Fallible<Weak<SyncSendEvent>> {
        let mut listeners = self.event_listeners.lock().unwrap();
        listeners.push(listener);
        Ok(Arc::downgrade(listeners.last().unwrap()))
    }

    pub fn remove_event_listener(&self, listener: &Weak<SyncSendEvent>) -> Fallible<()> {
        let listener = listener.upgrade();
        if listener.is_none() {
            return Ok(());
        }
        let listener = listener.unwrap();
        let mut listeners = self.event_listeners.lock().unwrap();
        let pos = listeners.iter().position(|x| Arc::ptr_eq(x, &listener));
        if let Some(idx) = pos {
            listeners.remove(idx);
        }

        Ok(())
    }

    /// Closes the target Page
    pub fn close_target(&self) -> Fallible<bool> {
        self.call_method(protocol::target::methods::CloseTarget {
            target_id: self.get_target_id(),
        })
        .map(|r| r.success)
    }

    /// Tries to close page, running its beforeunload hooks, if any
    pub fn close_with_unload(&self) -> Fallible<bool> {
        self.call_method(protocol::page::methods::Close {})
            .map(|_| true)
    }

    /// Calls one of the close_* methods depending on fire_unload option
    pub fn close(&self, fire_unload: bool) -> Fallible<bool> {
        self.optional_slow_motion_sleep(50);

        if fire_unload {
            return self.close_with_unload();
        }
        self.close_target()
    }

    /// Activates (focuses) the target.
    pub fn activate(&self) -> Fallible<&Self> {
        self.call_method(protocol::target::methods::ActivateTarget {
            target_id: self.get_target_id(),
        })
        .map(|_| self)
    }

    /// Get position and size of the browser window associated with this `Tab`.
    ///
    /// Note that the returned bounds are always specified for normal (windowed)
    /// state; they do not change when minimizing, maximizing or setting to
    /// fullscreen.
    pub fn get_bounds(&self) -> Result<protocol::browser::CurrentBounds, Error> {
        self.transport
            .call_method_on_browser(protocol::browser::methods::GetWindowForTarget {
                target_id: self.get_target_id(),
            })
            .map(|r| r.bounds.into())
    }

    /// Set position and/or size of the browser window associated with this `Tab`.
    ///
    /// When setting the window to normal (windowed) state, unspecified fields
    /// are left unchanged.
    pub fn set_bounds(&self, bounds: protocol::browser::Bounds) -> Result<&Self, Error> {
        let window_id = self
            .transport
            .call_method_on_browser(protocol::browser::methods::GetWindowForTarget {
                target_id: self.get_target_id(),
            })?
            .window_id;
        // If we set Normal window state, we *have* to make two API calls
        // to set the state before setting the coordinates; despite what the docs say...
        if let protocol::browser::Bounds::Normal { .. } = &bounds {
            self.transport
                .call_method_on_browser(protocol::browser::methods::SetWindowBounds {
                    window_id,
                    bounds: protocol::browser::methods::Bounds {
                        left: None,
                        top: None,
                        width: None,
                        height: None,
                        window_state: protocol::browser::WindowState::Normal,
                    },
                })?;
        }
        self.transport
            .call_method_on_browser(protocol::browser::methods::SetWindowBounds {
                window_id,
                bounds: bounds.into(),
            })?;
        Ok(self)
    }

    /// Returns all cookies that match the tab's current URL.
    pub fn get_cookies(&self) -> Fallible<Vec<Cookie>> {
        Ok(self
            .call_method(network::methods::GetCookies { urls: None })?
            .cookies)
    }

    /// Returns the title of the document.
    ///
    /// ```rust
    /// # use failure::Fallible;
    /// # use headless_chrome::Browser;
    /// # fn main() -> Fallible<()> {
    /// #
    /// # let browser = Browser::default()?;
    /// # let tab = browser.wait_for_initial_tab()?;
    /// tab.navigate_to("https://google.com")?;
    /// tab.wait_until_navigated()?;
    /// let title = tab.get_title()?;
    /// assert_eq!(title, "Google");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_title(&self) -> Fallible<String> {
        let remote_object = self.evaluate("document.title", false)?;
        Ok(serde_json::from_value(remote_object.value.unwrap())?)
    }

    /// If enabled, instead of using the GUI to select files, the browser will
    /// wait for the `Tab.handle_file_chooser` method to be called.
    /// **WARNING**: Only works on Chromium / Chrome 77 and above.
    pub fn set_file_chooser_dialog_interception(&self, enabled: bool) -> Fallible<()> {
        self.call_method(SetInterceptFileChooserDialog { enabled })?;
        Ok(())
    }

    /// Will have the same effect as choosing these files from the file chooser dialog that would've
    /// popped up had `set_file_chooser_dialog_interception` not been called. Calls to this method
    /// must be preceded by calls to that method.
    ///
    /// Supports selecting files or closing the file chooser dialog.
    ///
    /// NOTE: the filepaths listed in `files` must be absolute.
    pub fn handle_file_chooser(
        &self,
        action: FileChooserAction,
        files: Option<Vec<String>>,
    ) -> Fallible<()> {
        self.call_method(HandleFileChooser { action, files })?;
        Ok(())
    }
}
