#![allow(unused_variables)]

use headless_chrome::{
    browser::default_executable, protocol::page::ScreenshotFormat, Browser, LaunchOptionsBuilder,
    Tab,
};
use log::*;
use rand::prelude::*;
use std::sync::Arc;

mod logging;
mod server;

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

fn dumb_client(server: &server::Server) -> (Browser, Arc<Tab>) {
    let browser = Browser::new(
        LaunchOptionsBuilder::default()
            .path(Some(default_executable().unwrap()))
            .build()
            .unwrap(),
    )
    .unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))
        .unwrap();
    (browser, tab)
}

#[test]
fn simple() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    Ok(())
}

#[test]
fn actions_on_tab_wont_hang_after_browser_drops() -> Result<(), failure::Error> {
    logging::enable_logging();
    for _ in 0..20 {
        let (_, browser, tab) = dumb_server(include_str!("simple.html"));
        std::thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let millis: u64 = rng.gen_range(0, 5000);
            std::thread::sleep(std::time::Duration::from_millis(millis));
            trace!("dropping browser");
            drop(browser);
        });
        let _element = tab.find_element("div#foobar");
    }
    Ok(())
}

#[test]
fn form_interaction() -> Result<(), failure::Error> {
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

fn decode_png(i: &[u8]) -> Result<Vec<u8>, failure::Error> {
    let decoder = png::Decoder::new(&i[..]);
    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf)?;
    Ok(buf)
}

#[test]
fn capture_screenshot_png() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    // Check that the top-left pixel on the page has the background color set in simple.html
    let png_data = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
    let buf = decode_png(&png_data[..])?;
    assert_eq!(buf[0..4], [0x11, 0x22, 0x33, 0xff][..]);
    Ok(())
}

#[test]
fn capture_screenshot_element() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    // Check that the screenshot of the div's content-box has no other color than the one set in simple.html
    let png_data = tab
        .wait_for_element("div#foobar")?
        .capture_screenshot(ScreenshotFormat::PNG)?;
    let buf = decode_png(&png_data[..])?;
    for i in 0..buf.len() / 4 {
        assert_eq!(buf[i * 4..(i + 1) * 4], [0x33, 0x22, 0x11, 0xff][..]);
    }
    Ok(())
}

#[test]
fn capture_screenshot_element_box() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    // Check that the top-left pixel of the div's border-box has the border's color set in simple.html
    let pox = tab.wait_for_element("div#foobar")?.get_box_model()?;
    let png_data =
        tab.capture_screenshot(ScreenshotFormat::PNG, Some(pox.border_viewport()), true)?;
    let buf = decode_png(&png_data[..])?;
    assert_eq!(buf[0..4], [0x22, 0x11, 0x33, 0xff][..]);
    Ok(())
}

#[test]
fn capture_screenshot_jpeg() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    let jpg_data = tab.capture_screenshot(ScreenshotFormat::JPEG(Some(100)), None, true)?;
    let mut decoder = jpeg_decoder::Decoder::new(&jpg_data[..]);
    let buf = decoder.decode().unwrap();
    // Check that the total compression error is small-ish compared to the expected
    // pixel color
    let err = buf
        .iter()
        .zip(&[0x11, 0x22, 0x33])
        .map(|(b, e)| (i32::from(*b) - e).pow(2) as u32)
        .sum::<u32>();
    assert!(err < 5);
    Ok(())
}

#[test]
fn get_box_model() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_, browser, tab) = dumb_server(include_str!("simple.html"));
    let pox = tab.wait_for_element("div#foobar")?.get_box_model()?;
    // Check that the div has exactly the dimensions we set in simple.html
    assert_eq!(pox.width, 3 + 100 + 3);
    assert_eq!(pox.height, 3 + 20 + 3);
    Ok(())
}

#[test]
fn box_model_geometry() -> Result<(), failure::Error> {
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
fn reload() -> Result<(), failure::Error> {
    logging::enable_logging();
    let mut counter = 0;
    let responder = move |r: tiny_http::Request| {
        let response = tiny_http::Response::new(
            200.into(),
            vec![tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap()],
            std::io::Cursor::new(format!(r#"<div id="counter">{}</div>"#, counter)),
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
fn find_elements() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (server, browser, tab) = dumb_server(include_str!("simple.html"));
    let divs = tab.wait_for_elements("div")?;
    assert_eq!(8, divs.len());
    Ok(())
}

#[test]
fn incognito_contexts() -> Result<(), failure::Error> {
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
