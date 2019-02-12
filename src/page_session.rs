
//#[cfg(test)]
//mod tests {
//    use failure::{Error};
//    use crate::logging;
//
//    fn do_test() -> Result<(), Error> {
//        logging::enable_logging();
//        let chrome = crate::process::Process::new(crate::process::LaunchOptions{
//            headless: true,
//            ..Default::default()
//        })?;
//        let tab = chrome.new_tab()?;
//
//        tab.navigate_to("http://todomvc.com/examples/vanillajs/")?;
//        let element = tab.find_element("input")?;
//        element.click()?;
//        tab.type_str("buy cereal")?;
//        tab.press_key("Enter")?;
//        let todo_label = tab.wait_for_element("li label")?;
//        let children = todo_label.get_description()?.children.unwrap();
//        let text = &children.first().unwrap().node_value;
//        assert_eq!("buy cereal", text);
//        Ok(())
//    }
//
//    fn handles_remote_errors() -> Result<(), Error> {
//        logging::enable_logging();
//        let chrome = crate::process::Process::new(Default::default())?;
//        let tab = chrome.new_tab()?;
//        tab.navigate_to("http://todomvc.com/examples/vanillajs/")?;
//
//        // 0 is never a good node ID, AFAICT
//        let node_description = tab.describe_node(0);
//        assert_eq!(true, node_description.is_err());
//
//        let element = tab.find_element("a pretty terrible CSS selector");
//        assert_eq!(true, element.is_err());
//        Ok(())
//    }
//
//    #[test]
//    fn session_methods() {
//        handles_remote_errors().expect("worked");
//        do_test().expect("worked");
//    }
//}
