use std::sync::Arc;

use failure::Fallible;

use crate::browser::tab::Tab;
use crate::protocol::target::methods::CreateTarget;

/// Equivalent to a new incognito window
pub struct Context<'a> {
    id: String,
    browser: &'a super::Browser,
}

impl<'a> Context<'a> {
    pub fn new(browser: &'a super::Browser, context_id: String) -> Self {
        Self {
            id: context_id,
            browser,
        }
    }

    /// Opens a new tab in this context. It will not share cookies or a cache with the default
    /// browsing context or any other contexts created
    pub fn new_tab(&self) -> Fallible<Arc<Tab>> {
        let tab_in_context = CreateTarget {
            url: "about:blank",
            width: None,
            height: None,
            browser_context_id: Some(&self.id),
            enable_begin_frame_control: None,
        };
        self.browser.new_tab_with_options(tab_in_context)
    }

    /// The BrowserContextId associated with this context
    pub fn get_id(&self) -> &str {
        &self.id
    }

    /// Any tabs created in this context
    pub fn get_tabs(&self) -> Fallible<Vec<Arc<Tab>>> {
        let browser_tabs = self.browser.get_tabs().lock().unwrap();
        let mut tabs = vec![];
        for tab in browser_tabs.iter() {
            if let Some(context_id) = tab.get_browser_context_id()? {
                if context_id == self.id {
                    tabs.push(Arc::clone(tab));
                }
            }
        }
        Ok(tabs)
    }
}
