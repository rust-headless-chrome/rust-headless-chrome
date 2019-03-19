use std::fs;
use headless_chrome::{protocol::page::PrintToPdfOptions, Browser);


fn main() -> Result<(), failure::Error> {
    // Create a headless browser, navigate to index.html file, wait for the page
    // to render completely, render a PDF of the page and save it to the filesystem.

    // webSocketDebuggerUrl is obtained by GET http request to 
    // http://{chromeservicehost}{port}/json/version
    // and pass an empty-value host param: {host: ""} in header 
    let debug_ws_url = "ws://127.0.0.1:9222/devtools/browser/14804b82-0392-43be-b20f-d75678460e43";

    let browser = Browser::connect(debug_ws_url.to_string())?;
    let tab = browser.wait_for_initial_tab()?;

    let wikidata = tab
        .navigate_to("https://www.wikipedia.org")?
        .wait_until_navigated()?
        .print_to_pdf(None)?;
    fs::write("wiki.pdf", &wikidata)?;
    println!("PDF successfully created from internet web page.");

    // Browse to the file url and render a pdf of the web page.
    let pdf_options: Option<PrintToPdfOptions> = None;  // use chrome's defaults for this example
    let pdf_data = tab.navigate_to("file://index.html")?
                      .wait_until_navigated()?
                      .print_to_pdf(pdf_options)?;
    fs::write("rust.pdf", &pdf_data)?;
    println!("PDF successfully created from local web page.");

    Ok(())
}
