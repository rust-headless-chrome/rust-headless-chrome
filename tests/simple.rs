#![allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::Result;
use base64::Engine;
use headless_chrome::protocol::cdp::Browser::WindowState;
use headless_chrome::protocol::cdp::Fetch::events::RequestPausedEvent;
use headless_chrome::protocol::cdp::Fetch::{
    FulfillRequest, HeaderEntry, RequestPattern, RequestStage,
};
use headless_chrome::protocol::cdp::Network::{Cookie, CookieParam};
use headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption;
use headless_chrome::protocol::cdp::Runtime::{RemoteObjectSubtype, RemoteObjectType};
use headless_chrome::protocol::cdp::DOM::RGBA;
use headless_chrome::types::{Bounds, RemoteError};
use headless_chrome::LaunchOptionsBuilder;
use log::*;
use rand::prelude::*;

use headless_chrome::browser::tab::RequestPausedDecision;
use headless_chrome::browser::transport::{SessionId, Transport};
use headless_chrome::util::Wait;
use headless_chrome::{Browser, Tab};
use std::collections::HashMap;

pub mod logging;
pub mod server;

/// Launches a dumb server that unconditionally serves the given data as a
/// successful html response; launches a new browser and navigates to the
/// server.
///
/// Users must hold on to the server, which stops when dropped.
fn dumb_server(data: &'static str) -> (server::Server, Browser, Arc<Tab>) {
    let server = server::Server::with_dumb_html(data);
    let (browser, tab) = dumb_client(&server);
    (server, browser, tab)
}

fn browser() -> Browser {
    Browser::new(
        LaunchOptionsBuilder::default()
            .headless(true)
            .build()
            .unwrap(),
    )
    .unwrap()
}

fn dumb_client(server: &server::Server) -> (Browser, Arc<Tab>) {
    let browser = browser();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))
        .unwrap();
    (browser, tab)
}

#[test]
fn simple() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    Ok(())
}

// NOTE: we disable this test for Mac, because Travis doesn't support xvfb as a 'service'
#[cfg(target_os = "linux")]
#[test]
fn bounds_changed() -> Result<(), anyhow::Error> {
    logging::enable_logging();
    let browser = browser();
    let tab = browser.new_tab().unwrap();

    // New browser windows start in normal (windowed) state
    let bounds = tab.get_bounds()?;
    assert_eq!(bounds.state, WindowState::Normal);

    // Return to normal window size, setting only the coordinates
    tab.set_bounds(Bounds::Normal {
        left: Some(5),
        top: Some(5),
        width: None,
        height: None,
    })?;
    let new_bounds = tab.get_bounds()?;
    assert_eq!(new_bounds.state, WindowState::Normal);
    assert_eq!(new_bounds.left, 5);
    assert_eq!(new_bounds.top, 5);
    assert_eq!(new_bounds.width, bounds.width);
    assert_eq!(new_bounds.height, bounds.height);

    tab.set_bounds(Bounds::Normal {
        left: None,
        top: None,
        width: Some(200.0),
        height: Some(100.0),
    })?;
    let new_bounds = tab.get_bounds()?;
    assert_eq!(new_bounds.state, WindowState::Normal);
    assert_eq!(new_bounds.left, 5);
    assert_eq!(new_bounds.top, 5);
    assert_eq!(new_bounds.width, 200.0);
    assert_eq!(new_bounds.height, 100.0);
    Ok(())
}

#[test]
fn bounds_unchanged() -> Result<(), anyhow::Error> {
    logging::enable_logging();
    let browser = browser();
    let tab = browser.new_tab().unwrap();
    let bounds = tab.get_bounds()?;

    // Minimizing a window does *not* change it's bounds
    tab.set_bounds(Bounds::Minimized)?;
    let min_bounds = tab.get_bounds()?;
    assert_eq!(min_bounds.state, WindowState::Minimized);
    assert_eq!(min_bounds.left, bounds.left);
    assert_eq!(min_bounds.top, bounds.top);
    assert_eq!(min_bounds.width, bounds.width);
    assert_eq!(min_bounds.height, bounds.height);

    // Maximizing a window does *not* change it's bounds
    tab.set_bounds(Bounds::Maximized)?;
    let max_bounds = tab.get_bounds()?;
    assert_eq!(max_bounds.state, WindowState::Maximized);
    assert_eq!(max_bounds.left, bounds.left);
    assert_eq!(max_bounds.top, bounds.top);
    assert!(max_bounds.width >= bounds.width);
    assert!(max_bounds.height >= bounds.height);

    // Setting a window to fullscreen does *not* change it's bounds
    tab.set_bounds(Bounds::Fullscreen)?;
    let fs_bounds = tab.get_bounds()?;
    assert_eq!(fs_bounds.state, WindowState::Fullscreen);
    assert_eq!(fs_bounds.left, bounds.left);
    assert_eq!(fs_bounds.top, bounds.top);
    assert!(fs_bounds.width >= bounds.width);
    assert!(fs_bounds.height >= bounds.height);

    Ok(())
}

#[test]
fn actions_on_tab_wont_hang_after_browser_drops() -> Result<()> {
    logging::enable_logging();
    for _ in 0..20 {
        let (_, browser, tab) = dumb_server(include_str!("simple.html"));
        std::thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let millis: u64 = rng.gen_range(0..5000);
            std::thread::sleep(std::time::Duration::from_millis(millis));
            trace!("dropping browser");
            drop(browser);
        });
        let _element = tab.find_element("div#foobar");
    }
    Ok(())
}

#[test]
fn form_interaction() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("form.html"));
    tab.wait_for_element("input#target")?
        .type_into("mothership")?;
    tab.wait_for_element("button")?.click()?;
    let d = tab.wait_for_element("div#protocol")?.get_description()?;
    assert!(d
        .find(|n| n.node_value == "Missiles launched against mothership")
        .is_some());
    tab.wait_for_element("input#sneakattack")?.click()?;
    tab.wait_for_element("button")?.click()?;
    let d = tab.wait_for_element("div#protocol")?.get_description()?;
    assert!(d
        .find(|n| n.node_value == "Comrades, have a nice day!")
        .is_some());
    Ok(())
}

#[test]
fn send_character() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("form.html"));
    tab.find_element("input#target")?.click()?;
    tab.send_character("mothership")?;
    tab.find_element("button")?.click()?;
    let d = tab.wait_for_element("div#protocol")?.get_description()?;
    assert!(d
        .find(|n| n.node_value == "Missiles launched against mothership")
        .is_some());

    Ok(())
}

#[test]
fn tab_get_content() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    let html = tab.get_content()?;
    // The html returned depends on how the browser formatted it. The HTML is always correct, but
    // some of the newlines or tabs might be missing.
    assert!(html.replace('\n', "") == include_str!("simple.html").replace('\n', ""));
    Ok(())
}

#[test]
fn element_get_content() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    let elem = tab.find_element("div#within")?;
    let html = elem.get_content()?;
    assert!(html == r#"<div id="within"></div>"#);
    Ok(())
}

fn decode_png(i: &[u8]) -> Result<Vec<u8>> {
    let decoder = png::Decoder::new(i);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    reader.next_frame(&mut buf)?;
    Ok(buf)
}

fn sum_of_errors(inp: &[u8], fixture: &[u8]) -> u32 {
    inp.chunks_exact(fixture.len())
        .map(|c| {
            c.iter()
                .zip(fixture)
                .map(|(b, e)| (i32::from(*b) - i32::from(*e)).pow(2) as u32)
                .sum::<u32>()
        })
        .sum()
}

#[test]
fn set_background_color() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("transparent.html"));
    tab.wait_for_element("body")?;
    // Check that the top-left pixel on the page has the background color set in transparent.html
    tab.set_background_color(RGBA {
        r: 255,
        g: 0,
        b: 0,
        a: Some(1.),
    })?;
    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
    let buf = decode_png(&png_data[..])?;
    assert!(sum_of_errors(&buf[0..4], &[0xff, 0x00, 0x00, 0xff]) < 5);
    Ok(())
}

#[test]
fn set_transparent_background_color() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("transparent.html"));
    tab.wait_for_element("body")?;
    // Check that the top-left pixel on the page has the background color set in transparent.html
    tab.set_transparent_background_color()?;
    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
    let buf = decode_png(&png_data[..])?;
    assert!(sum_of_errors(&buf[0..4], &[0x00, 0x00, 0x00, 0x00]) < 5);
    Ok(())
}

#[test]
fn capture_screenshot_png() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    // Check that the top-left pixel on the page has the background color set in simple.html
    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;
    let buf = decode_png(&png_data[..])?;
    assert!(sum_of_errors(&buf[0..4], &[0x11, 0x22, 0x33, 0xff]) < 5);
    Ok(())
}

#[test]
fn capture_screenshot_element() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    // Check that the screenshot of the div's content-box has no other color than the one set in simple.html
    let png_data = tab
        .wait_for_element("div#foobar")?
        .capture_screenshot(CaptureScreenshotFormatOption::Png)?;
    let buf = decode_png(&png_data[..])?;
    for i in 0..buf.len() / 4 {
        assert!(sum_of_errors(&buf[i * 4..(i + 1) * 4], &[0x33, 0x22, 0x11, 0xff]) < 5);
    }
    Ok(())
}

#[test]
fn capture_screenshot_element_box() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    // Check that the top-left pixel of the div's border-box has the border's color set in simple.html
    let pox = tab.wait_for_element("div#foobar")?.get_box_model()?;
    let png_data = tab.capture_screenshot(
        CaptureScreenshotFormatOption::Png,
        None,
        Some(pox.border_viewport()),
        true,
    )?;
    let buf = decode_png(&png_data[..])?;
    assert!(dbg!(sum_of_errors(&buf[0..4], &[0x22, 0x11, 0x33, 0xff])) < 15);
    Ok(())
}

#[test]
fn capture_screenshot_jpeg() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    let jpg_data =
        tab.capture_screenshot(CaptureScreenshotFormatOption::Jpeg, Some(100), None, true)?;
    let mut decoder = jpeg_decoder::Decoder::new(&jpg_data[..]);
    let buf = decoder.decode().unwrap();
    assert!(sum_of_errors(&buf[0..4], &[0x11, 0x22, 0x33]) < 5);
    Ok(())
}

#[test]
fn test_print_file_to_pdf() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("./pdfassets/index.html"));
    let local_pdf = tab.wait_until_navigated()?.print_to_pdf(None)?;
    assert!(local_pdf.len() > 1000); // an arbitrary size
    assert!(local_pdf.starts_with(b"%PDF"));
    Ok(())
}

#[test]
fn get_box_model() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    let pox = tab.wait_for_element("div#foobar")?.get_box_model()?;
    // Check that the div has exactly the dimensions we set in simple.html
    assert_eq!(pox.width as i32, 3 + 100 + 3);
    assert_eq!(pox.height as i32, 3 + 20 + 3);
    Ok(())
}

#[test]
fn box_model_geometry() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    let center = tab.wait_for_element("div#position-test")?.get_box_model()?;
    let within = tab.wait_for_element("div#within")?.get_box_model()?;
    let above = tab
        .wait_for_element("div#strictly-above")?
        .get_box_model()?;
    let below = tab
        .wait_for_element("div#strictly-below")?
        .get_box_model()?;
    let left = tab.wait_for_element("div#strictly-left")?.get_box_model()?;
    let right = tab
        .wait_for_element("div#strictly-right")?
        .get_box_model()?;

    assert!(above.content.strictly_above(&center.content));
    assert!(above.content.above(&center.content));
    assert!(above.margin.above(&center.content));
    assert!(!above.margin.strictly_above(&center.content));
    assert!(above.content.within_horizontal_bounds_of(&center.content));
    assert!(!above.content.within_vertical_bounds_of(&center.content));

    assert!(below.content.strictly_below(&center.content));
    assert!(below.content.below(&center.content));
    assert!(below.margin.below(&center.content));
    assert!(!below.margin.strictly_below(&center.content));

    assert!(left.content.strictly_left_of(&center.content));
    assert!(left.content.left_of(&center.content));
    assert!(left.margin.left_of(&center.content));
    assert!(!left.margin.strictly_left_of(&center.content));
    assert!(!left.content.within_horizontal_bounds_of(&center.content));
    assert!(left.content.within_vertical_bounds_of(&center.content));

    assert!(right.content.strictly_right_of(&center.content));
    assert!(right.content.right_of(&center.content));
    assert!(right.margin.right_of(&center.content));
    assert!(!right.margin.strictly_right_of(&center.content));

    assert!(within.content.within_bounds_of(&center.content));
    assert!(!center.content.within_bounds_of(&within.content));

    Ok(())
}

#[test]
fn reload() -> Result<()> {
    logging::enable_logging();
    let mut counter = 0;
    let responder = move |r: tiny_http::Request| {
        let response = tiny_http::Response::new(
            200.into(),
            vec![tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()],
            std::io::Cursor::new(format!(r#"<div id="counter">{counter}</div>"#)),
            None,
            None,
        );
        trace!("{}", counter);
        counter += 1;
        r.respond(response)
    };
    let server = server::Server::new(responder);
    let (browser, tab) = dumb_client(&server);
    assert!(tab
        .wait_for_element("div#counter")?
        .get_description()?
        .find(|n| n.node_value == "0")
        .is_some());
    assert!(tab
        .reload(false, None)?
        .wait_for_element("div#counter")?
        .get_description()?
        .find(|n| n.node_value == "1")
        .is_some());
    // TODO test effect of scriptEvaluateOnLoad
    Ok(())
}

#[test]
fn find_elements() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let divs = tab.wait_for_elements("div")?;
    let divs_xpath = tab.wait_for_elements_by_xpath("//*[@id]")?;
    assert_eq!(8, divs.len());
    assert_eq!(7, divs_xpath.len());
    Ok(())
}

/*
#[test]
fn find_element_on_tab_and_other_elements() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let containing_element = tab.find_element("div#position-test")?;
    let inner_element = containing_element.find_element("#strictly-above")?;
    dbg!(&inner_element);
    let attrs = inner_element.get_attributes()?.unwrap();
    assert_eq!(attrs["id"], "strictly-above");
    Ok(())
}
*/

#[test]
fn find_element_on_tab_by_xpath() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let containing_element_xpath = tab.wait_for_xpath("/html/body/div[2]")?;
    let inner_element_xpath =
        containing_element_xpath.wait_for_xpath(r#"//*[@id="strictly-above"]"#)?;
    dbg!(&inner_element_xpath);
    let attrs = inner_element_xpath.get_attributes()?.unwrap();
    let id: Vec<&String> = attrs.iter().filter(|v| v.as_str() == "id").collect();
    assert_eq!(id[0].as_str(), "strictly-above");

    Ok(())
}

#[test]
fn set_user_agent() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(
        r#"
<html>
<body>
<script>
document.write(navigator.userAgent + ";" + navigator.platform + ";" + navigator.language);
</script>
</body>
</html>
"#,
    );
    tab.set_user_agent("UnitTestClient", Some("de-DE-1996"), Some("UnitTest"))?;
    // The test-tab has already navigated once, so reload to ensure that js
    // environment is using the correct values.
    tab.reload(true, None)?;
    assert!(tab
        .wait_for_element("body")?
        .get_description()?
        .find(|n| n
            .node_value
            .starts_with("UnitTestClient;UnitTest;de-DE-1996"))
        .is_some());
    Ok(())
}

#[test]
fn wait_for_element_returns_unexpected_errors_early() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let start = Instant::now();
    let remote_error = tab
        .wait_for_element("") // pass an invalid selector
        .unwrap_err()
        .downcast::<RemoteError>()?;
    assert_eq!(remote_error.message, "DOM Error while querying");
    assert!(start.elapsed() < Duration::from_secs(1));
    Ok(())
}

#[test]
fn call_js_fn_sync() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let element = tab.wait_for_element("#foobar")?;
    let result = element.call_js_fn("function() { return 42 }", vec![], false)?;
    assert_eq!(result.Type, RemoteObjectType::Number);
    assert_eq!(result.description, Some("42".to_owned()));
    assert_eq!(result.value, Some((42).into()));
    Ok(())
}

#[test]
fn call_js_fn_async_unresolved() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let element = tab.wait_for_element("#foobar")?;
    let result = element.call_js_fn("async function() { return 42 }", vec![], false)?;
    assert_eq!(result.Type, RemoteObjectType::Object);
    assert_eq!(result.subtype, Some(RemoteObjectSubtype::Promise));
    assert_eq!(result.description, Some("Promise".to_owned()));
    assert_eq!(result.value, None);
    Ok(())
}

#[test]
fn call_js_fn_async_resolved() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let element = tab.wait_for_element("#foobar")?;
    let result = element.call_js_fn("async function() { return 42 }", vec![], true)?;
    assert_eq!(result.Type, RemoteObjectType::Number);
    assert_eq!(result.subtype, None);
    assert_eq!(result.description, Some("42".to_owned()));
    assert_eq!(result.value, Some((42).into()));
    Ok(())
}

#[test]
fn evaluate_sync() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let result = tab.evaluate("(function () { return 42 })();", false)?;
    assert_eq!(result.Type, RemoteObjectType::Number);
    assert_eq!(result.description, Some("42".to_owned()));
    assert_eq!(result.value, Some((42).into()));
    Ok(())
}

#[test]
fn evaluate_async_unresolved() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let result = tab.evaluate("(async function () { return 42 })();", false)?;
    assert_eq!(result.Type, RemoteObjectType::Object);
    assert_eq!(result.description, Some("Promise".to_owned()));
    assert_eq!(result.subtype, Some(RemoteObjectSubtype::Promise));
    assert_eq!(result.value, None);
    Ok(())
}

#[test]
fn evaluate_async_resolved() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let result = tab.evaluate("(async function () { return 42 })();", true)?;
    assert_eq!(result.Type, RemoteObjectType::Number);
    assert_eq!(result.subtype, None);
    assert_eq!(result.description, Some("42".to_owned()));
    assert_eq!(result.value, Some((42).into()));
    Ok(())
}

#[test]
fn set_request_interception() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!(
        "coverage_fixtures/basic_page_with_js_scripts.html"
    ));

    let patterns = vec![
        RequestPattern {
            url_pattern: None,
            resource_Type: None,
            request_stage: Some(RequestStage::Response),
        },
        RequestPattern {
            url_pattern: None,
            resource_Type: None,
            request_stage: Some(RequestStage::Request),
        },
    ];
    tab.enable_fetch(Some(&patterns), None)?;

    tab.enable_request_interception(Arc::new(
        move |transport: Arc<Transport>, session_id: SessionId, intercepted: RequestPausedEvent| {
            if intercepted.params.request.url.ends_with(".js") {
                let js_body = r#"document.body.appendChild(document.createElement("hr"));"#;

                let headers = vec![HeaderEntry {
                    name: "Content-Type".to_string(),
                    value: "application/javascript".to_string(),
                }];

                let fulfill_request = FulfillRequest {
                    request_id: intercepted.params.request_id,
                    response_code: 200,
                    response_headers: Some(headers),
                    binary_response_headers: None,
                    body: Some(base64::prelude::BASE64_STANDARD.encode(js_body)),
                    response_phrase: None,
                };

                RequestPausedDecision::Fulfill(fulfill_request)
            } else {
                RequestPausedDecision::Continue(None)
            }
        },
    ))?;

    // ignore cache:

    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))
        .unwrap();
    tab.wait_until_navigated()?;
    // There are two JS scripts that get loaded via network, they both append an element like this:
    assert_eq!(2, tab.wait_for_elements("hr")?.len());

    Ok(())
}

#[test]
fn authentication() -> Result<()> {
    logging::enable_logging();
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.authenticate(Some("login".to_string()), Some("password".to_string()))?;
    tab.enable_fetch(None, Some(true))?;
    tab.navigate_to("http://httpbin.org/basic-auth/login/password")?;
    tab.wait_until_navigated()?;

    Ok(())
}

#[test]
fn response_handler() -> Result<()> {
    logging::enable_logging();
    let server = server::Server::with_dumb_html(include_str!(
        "coverage_fixtures/basic_page_with_js_scripts.html"
    ));

    let browser = Browser::default()?;

    let tab = browser.new_tab().unwrap();

    let responses = Arc::new(Mutex::new(Vec::new()));

    let responses2 = responses.clone();
    let responses3 = responses.clone();
    assert!(tab
        .register_response_handling(
            "test1",
            Box::new(move |response, fetch_body| {
                // NOTE: you can only fetch the body after it's been downloaded, which might be some time
                // after the initial 'response' (with status code, headers, etc.) has come back. hence this
                // sleep:
                sleep(Duration::from_millis(500));
                let body = fetch_body().unwrap();
                responses2.lock().unwrap().push((response, body));
            })
        )?
        .is_none());

    assert!(tab
        .register_response_handling(
            "test2",
            Box::new(move |response, fetch_body| {
                // NOTE: you can only fetch the body after it's been downloaded, which might be some time
                // after the initial 'response' (with status code, headers, etc.) has come back. hence this
                // sleep:
                sleep(Duration::from_millis(500));
                let body = fetch_body().unwrap();
                responses3.lock().unwrap().push((response, body));
            })
        )?
        .is_none());

    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))
        .unwrap();

    tab.wait_until_navigated()?;

    let final_responses: Vec<_> = responses.lock().unwrap().clone();
    assert_eq!(final_responses.len(), 3 * 2);
    assert_eq!(final_responses[0].0.response.mime_type, "text/html");
    assert!(final_responses[0].1.body.contains("Click me"));

    Ok(())
}

#[test]
fn loading_failed_handler() -> Result<()> {
    logging::enable_logging();
    let server = server::Server::with_dumb_html(include_str!(
        "coverage_fixtures/basic_page_with_loading_failed_element.html"
    ));
    let browser = Browser::default()?;

    let tab = browser.new_tab().unwrap();

    let failed_event = Arc::new(Mutex::new(Vec::new()));

    let failed_event_clone = failed_event.clone();
    assert!(tab
        .register_loading_failed_handling(
            "test1",
            Box::new(move |response, loading_failed| {
                failed_event_clone
                    .lock()
                    .unwrap()
                    .push((response, loading_failed))
            })
        )?
        .is_none());

    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))
        .unwrap();

    tab.wait_until_navigated()?;

    let final_failed_event: Vec<_> = failed_event.lock().unwrap().clone();
    assert_eq!(final_failed_event.len(), 1);
    assert_eq!(
        final_failed_event[0].0.response.url,
        "http://httpbin.org/status/404"
    );
    assert_eq!(final_failed_event[0].0.response.status, 404);
    assert_eq!(final_failed_event[0].1.error_text, "net::ERR_ABORTED");

    Ok(())
}

#[test]
fn incognito_contexts() -> Result<()> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));

    let incognito_context = browser.new_context()?;
    let incognito_tab: Arc<Tab> = incognito_context.new_tab()?;
    let tab_context_id = incognito_tab.get_target_info()?.browser_context_id.unwrap();

    assert_eq!(incognito_context.get_id(), tab_context_id);
    assert_eq!(
        incognito_context.get_tabs()?[0].get_target_id(),
        incognito_tab.get_target_id()
    );
    Ok(())
}

#[test]
fn get_script_source() -> Result<()> {
    logging::enable_logging();
    let server = server::file_server("tests/coverage_fixtures");
    let browser = Browser::default()?;

    let tab: Arc<Tab> = browser.new_tab()?;

    tab.enable_profiler()?;
    tab.start_js_coverage()?;

    tab.navigate_to(&format!(
        "{}/basic_page_with_js_scripts.html",
        &server.url()
    ))?;

    tab.wait_until_navigated()?;

    sleep(Duration::from_millis(500));

    let script_coverages = tab.take_precise_js_coverage()?;

    tab.enable_debugger()?;

    let contents = tab.get_script_source(&script_coverages[0].script_id)?;
    assert_eq!(
        include_str!("coverage_fixtures/coverage_fixture1.js"),
        contents
    );

    let contents = tab.get_script_source(&script_coverages[1].script_id)?;
    assert_eq!(
        include_str!("coverage_fixtures/coverage_fixture2.js"),
        contents
    );

    Ok(())
}

#[test]
fn read_write_cookies() -> Result<()> {
    logging::enable_logging();
    let responder = move |r: tiny_http::Request| {
        let response = tiny_http::Response::new(
            200.into(),
            vec![
                tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap(),
                tiny_http::Header::from_bytes(&b"Set-Cookie"[..], &b"testing=1; Max-Age=100;"[..])
                    .unwrap(),
            ],
            std::io::Cursor::new("<div></div>"),
            None,
            None,
        );
        r.respond(response)
    };
    let server = server::Server::new(responder);
    let (browser, tab) = dumb_client(&server);

    // can read cookies
    {
        tab.navigate_to(&server.url())?;

        tab.wait_until_navigated()?;

        let cookies = tab.get_cookies()?;

        assert_eq!(cookies.len(), 1);

        let Cookie { name, value, .. } = cookies.first().unwrap();

        assert_eq!(name, "testing");
        assert_eq!(value, "1");

        let t: Result<()> = Ok(()); // type hint for error
        t
    }?;

    // can change (delete and set) value for current url
    {
        tab.set_cookies(vec![CookieParam {
            name: "testing".to_string(),
            value: "2".to_string(),
            url: None,
            domain: None,
            path: None,
            secure: None,
            http_only: None,
            same_site: None,
            expires: None,
            priority: None,
            same_party: None,
            source_scheme: None,
            source_port: None,
            partition_key: None,
        }])?;

        let cookies = tab.get_cookies()?;
        assert_eq!(cookies.len(), 1);
        let cf = cookies.first().unwrap();
        assert_eq!(cf.name, "testing");
        assert_eq!(cf.value, "2");

        let t: Result<()> = Ok(()); // type hint for error
        t
    }?;

    Ok(())
}

#[test]
fn close_tabs() -> Result<()> {
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let tabs = browser.get_tabs();

    let wait = Wait::with_timeout(Duration::from_secs(30));

    let check_tabs = |num: usize| {
        let num_tabs = tabs.lock().unwrap().len();
        assert_eq!(num, num_tabs);
    };
    let wait_tabs = |num: usize| {
        let num_tabs = tabs.lock().unwrap().len();
        if num_tabs == num {
            Some(true)
        } else {
            None
        }
    };

    check_tabs(1);

    let new_tab1 = browser.new_tab()?;
    new_tab1
        .navigate_to(&format!("http://127.0.0.1:{}", server.port()))?
        .wait_until_navigated()?;
    check_tabs(2);

    let new_tab2 = browser.new_tab()?;
    new_tab2
        .navigate_to(&format!("http://127.0.0.1:{}", server.port()))?
        .wait_until_navigated()?;

    check_tabs(3);

    let closed = new_tab1.close(true)?;

    assert!(closed);

    wait.until(|| wait_tabs(2))?;
    check_tabs(2);

    new_tab2.close_with_unload()?;
    wait.until(|| wait_tabs(1))?;
    check_tabs(1);

    Ok(())
}

#[test]
fn parses_shadow_doms() -> Result<()> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("shadow-dom.html"));
    tab.wait_for_element("html")?;
    Ok(())
}

#[test]
fn set_extra_http_headers() -> Result<()> {
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let mut headers = HashMap::new();
    headers.insert("test", "header");
    tab.set_extra_http_headers(headers)?;
    tab.enable_fetch(None, None)?;

    tab.enable_request_interception(Arc::new(
        |transport: Arc<Transport>, session_id: SessionId, intercepted: RequestPausedEvent| {
            println!("{:?}", intercepted.params.request.headers);
            // assert_eq!(
            //     intercepted.params.request.headers.get("test"),
            //     Some(&"header".to_string())
            // );
            RequestPausedDecision::Continue(None)
        },
    ))?;

    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))?
        .wait_until_navigated()?;
    Ok(())
}

#[test]
fn get_css_styles() -> Result<()> {
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));

    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))?
        .wait_until_navigated()?;

    let element = tab.wait_for_element("#within")?;

    let styles = element.get_computed_styles()?;

    let v = styles
        .iter()
        .filter_map(|p| {
            if p.name == "top" || p.name == "background-color" || p.name == "position" {
                Some(p.value.as_str())
            } else {
                None
            }
        })
        .collect::<Vec<&str>>();

    assert!(!v.is_empty());

    assert_eq!(["rgb(255, 255, 0)", "absolute", "5px"], v.as_slice());

    Ok(())
}
