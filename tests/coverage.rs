use std::sync::Arc;

use failure::Fallible;

use headless_chrome::browser::tab::Tab;
use headless_chrome::Browser;
use server::Server;

mod logging;
pub mod server;

fn basic_http_response(
    body: &'static str,
    content_type: &'static str,
) -> tiny_http::Response<&'static [u8]> {
    tiny_http::Response::new(
        200.into(),
        vec![tiny_http::Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap()],
        body.as_bytes(),
        Some(body.len()),
        None,
    )
}

fn server_with_html_and_js() -> Server {
    Server::new(|request: tiny_http::Request| {
        let url = request.url();

        let content_type = if url.ends_with(".js") {
            "application/javascript"
        } else {
            "text/html"
        };

        let body = if url.ends_with("coverage_fixture1.js") {
            include_str!("coverage_fixtures/coverage_fixture1.js")
        } else if url.ends_with("coverage_fixture2.js") {
            include_str!("coverage_fixtures/coverage_fixture2.js")
        } else {
            include_str!("coverage_fixtures/basic_page_with_js_scripts.html")
        };

        let response = basic_http_response(body, content_type);
        request.respond(response)
    })
}

#[test]
fn returns_actual_coverage() -> Fallible<()> {
    logging::enable_logging();
    let server = server_with_html_and_js();
    let browser = Browser::default()?;
    let tab: Arc<Tab> = browser.wait_for_initial_tab()?;

    tab.enable_profiler()?;
    tab.start_js_coverage()?;

    let url = format!("http://127.0.0.1:{}", server.port());
    tab.navigate_to(&url)?;

    tab.wait_until_navigated()?;

    let script_coverages = tab.take_precise_js_coverage()?;

    dbg!(&script_coverages);

    // our fixtures HTML file (basic_page_with_js_scripts.html) includes two external scripts
    assert_eq!(2, script_coverages.len());

    let first_script = script_coverages
        .iter()
        .find(|script_coverage| script_coverage.url.ends_with("coverage_fixture1.js"))
        .unwrap();

    // in coverage_fixture1.js, there are two anonymous functions and one nameless "function" that
    // you can imagine as being the global scope (i.e. the top level scope of the script)
    assert_eq!(3, first_script.functions.len());

    // this one should not have been executed yet:

    let on_click_function_coverage = first_script
        .functions
        .iter()
        .find(|function_coverage| function_coverage.function_name == "button.onclick")
        .unwrap();

    let fn_range = on_click_function_coverage.ranges.first().unwrap();
    assert_eq!(fn_range.count, 0);

    let button = tab.wait_for_element("#incrementor")?;
    button.click()?;

    let result = tab.take_precise_js_coverage()?;

    let updated_script_coverages: Vec<_> = result
        .iter()
        // discludes 'anonymous' scripts we inject into the page
        .filter(|script_cov| script_cov.url.starts_with("http://"))
        .collect();

    dbg!(&updated_script_coverages);

    // when we clicked the button, code in only one of the scripts was executed, and Chrome
    // only sends back info about code that's been executed since you last "took" the coverage:
    assert_eq!(1, updated_script_coverages.len());

    let updated_on_click_function_coverage = updated_script_coverages
        .first()
        .unwrap()
        .functions
        .first()
        .unwrap();

    assert_eq!(
        "button.onclick",
        updated_on_click_function_coverage.function_name
    );
    assert_eq!(
        1,
        updated_on_click_function_coverage
            .ranges
            .first()
            .unwrap()
            .count
    );

    Ok(())
}
