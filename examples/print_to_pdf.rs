use headless_chrome::{protocol::page::PrintToPdfOptions, Browser};
use std::fs;


fn main() -> Result<(), failure::Error> {
    // Create a headless browser, navigate to index.html file, wait for the page
    // to render completely, render a PDF of the page and save it to the filesystem.

    // webSocketDebuggerUrl is obtained by GET http request to 
    // http://{chromeservicehost}{port}/json/version
    // and pass an empty-value host param: {host: ""} in header 
    let debug_ws_url = "ws://localhost:9222/devtools/browser/7e2d7ecb-4c63-49de-9c06-7cd9ffcdd224";

    let browser = Browser::connect(debug_ws_url.to_string())?;
    let tab = browser.wait_for_initial_tab()?;

    // Browse to the file url and render a pdf of the web page.
    let pdf_options = None;  // use chrome's defaults for this example

    let pdf_data = tab
        .navigate_to("file://index.html")?
        .wait_until_navigated()?
        .print_to_pdf(pdf_options)?;
    fs::write("rust.pdf", &pdf_data)?;

    println!("PDF successfully created.");
    Ok(())
}
