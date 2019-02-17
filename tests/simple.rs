use headless_chrome::{browser, logging, process, tab};
use std::sync::Arc;
mod server;

/// Launches a dumb server that unconditionally serves the given data as a
/// successful html response; launches a new browser and navigates to the
/// server.
///
/// Users must hold on to the server, which stops when dropped.
fn dumb_server(data: &'static str) -> (server::Server, browser::Browser, Arc<tab::Tab>) {
    let server = server::Server::with_dumb_html(data);
    let browser = browser::Browser::new(process::LaunchOptions::default().unwrap()).unwrap();
    let tab = browser.wait_for_initial_tab().unwrap();
    tab.navigate_to(&format!("http://127.0.0.1:{}", server.port()))
        .unwrap();
    (server, browser, tab)
}

#[test]
fn simple() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_server, _browser, tab) = dumb_server(include_str!("simple.html"));
    tab.wait_for_element("div#foobar")?;
    Ok(())
}

#[test]
fn form_interaction() -> Result<(), failure::Error> {
    logging::enable_logging();
    let (_server, _browser, tab) = dumb_server(include_str!("form.html"));
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
