use std::fs::File;

use failure::Fallible;
use filepath::FilePath;
use regex::Regex;

use headless_chrome::protocol::page::methods::FileChooserAction;
use headless_chrome::Browser;

pub mod logging;
mod server;

#[test]
fn file_chooser_works() -> Fallible<()> {
    logging::enable_logging();
    let browser = Browser::default()?;
    let version = browser.get_version()?;
    let version_number_re: Regex = Regex::new(r"HeadlessChrome/(\d+)\.").unwrap();
    let version_number: u8 = version_number_re.captures(&version.product).unwrap()[1]
        .parse()
        .unwrap();

    if version_number > 76 {
        let tab = browser.wait_for_initial_tab()?;
        let server =
            server::Server::with_dumb_html(include_str!("file_chooser_fixtures/chooser.html"));

        tab.navigate_to(&server.url())?;

        tab.set_file_chooser_dialog_interception(true)?;

        let file_upload_button = tab.wait_for_element("input[type='file']")?;
        file_upload_button.click()?;

        // uses 'filepath' crate to get absolute path
        let test_file = File::open("tests/file_chooser_fixtures/file_to_upload.txt")?;
        let path = test_file.path()?;

        tab.handle_file_chooser(
            FileChooserAction::Accept,
            Some(vec![path.to_string_lossy().into_owned()]),
        )?;

        let eval_result =
            file_upload_button.call_js_fn("function() { return this.files[0]; }", false)?;
        let size_from_js: u64 = eval_result
            .preview
            .unwrap()
            .properties
            .iter()
            .find_map(|property_preview| {
                if property_preview.name == "size" {
                    Some(property_preview.value.as_ref().unwrap())
                } else {
                    None
                }
            })
            .unwrap()
            .parse()?;

        assert_eq!(size_from_js, test_file.metadata()?.len());
    }

    Ok(())
}
