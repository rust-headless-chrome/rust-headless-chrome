use anyhow::Result;

use headless_chrome::{Browser, LaunchOptions};

fn main() -> Result<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://www.google.com")
        .expect("failed to navigate");

    tab.wait_until_navigated().unwrap();

    let element = tab.wait_for_xpath("/html/body/div[1]/div[5]/div[1]")?;

    element.call_js_fn(
        "function(...args) {if(arg[0]) {return this.innerText;}}",
        vec![],
        false,
    )?;

    Ok(())
}
