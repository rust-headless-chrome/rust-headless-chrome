use std::{env, fs};

use failure::Fallible;

use headless_chrome::{protocol::page::PrintToPdfOptions, Browser};

/// Run the example as follows:
/// ./print_to_pdf {debug_ws_url} /rust-headless-chrome/tests/pdfassets/index.html
fn main() -> Fallible<()> {
    // Create a headless browser, navigate to index.html file, wait for the page
    // to render completely, render a PDF of the page and save it to the filesystem.

    // webSocketDebuggerUrl is obtained by GET http request to
    // http://{chromeservicehost}{port}/json/version
    // and pass an empty-value host param: {host: ""} in header
    //
    // an example looks like this: "ws://127.0.0.1:9222/devtools/browser/14804b82-0392-43be-b20f-d75678460e43";
    let debug_ws_url = env::args().nth(1).expect("Must provide debug_ws_url");

    let file_path = format!(
        "file://{}",
        env::args()
            .nth(2)
            .expect("Must provide path/to/file/index.html")
    );

    let browser = Browser::connect(debug_ws_url.to_string())?;
    let tab = browser.wait_for_initial_tab()?;

    let wikidata = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .print_to_pdf(None)?;
    fs::write("wiki.pdf", &wikidata)?;
    println!("PDF successfully created from internet web page.");

    // Browse to the file url and render a pdf of the web page.
    let pdf_options: Option<PrintToPdfOptions> = None; // use chrome's defaults for this example
    let local_pdf = tab
        .navigate_to(&file_path)?
        .wait_until_navigated()?
        .print_to_pdf(pdf_options)?;
    fs::write("rust.pdf", &local_pdf)?;
    println!("PDF successfully created from local web page.");

    Ok(())
}
