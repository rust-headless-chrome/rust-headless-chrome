use failure::Fallible;

use headless_chrome::{Browser, LaunchOptions};

fn query(input: &str) -> Fallible<()> {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .build()
            .expect("Could not find chrome-executable"),
    )?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("https://en.wikipedia.org")?
        .wait_for_element("input#searchInput")?
        .click()?;
    tab.type_str(&input)?.press_key("Enter")?;
    match tab.wait_for_element("div.shortdescription") {
        Err(e) => eprintln!("Query failed: {:?}", e),
        Ok(e) => match e.get_description()?.find(|n| n.node_name == "#text") {
            Some(n) => println!("Result for `{}`: {}", &input, n.node_value),
            None => eprintln!("No shortdescription-node found on page"),
        },
    }
    Ok(())
}

fn main() -> Fallible<()> {
    let input = "Elvis Aaron Presley";
    query(input)
}
