use std::sync::Arc;

use crate::{
    browser::transport::{SessionId, Transport},
    protocol::cdp::Page::HandleJavaScriptDialog,
};
use anyhow::Result;

/// A handle to an instance of a dialog opened in a tab.
///
/// You can handle any dialog opened by `alert`, `confirm`, or `prompt`.
/// ```rust
/// # fn example(tab: headless_chrome::browser::tag::Tab) -> anyhow::Result<()> {
/// #
/// use headless_chrome::protocol::cdp::types::Event;
/// tab.add_event_listener(Arc::new({
///   let tab = tab.clone();
///   move |event: &Event| match event {
///     Event::PageJavascriptDialogOpening(event) => {
///         let dialog = tab.get_dialog();
///         let _ = if event.params.message == "Please accept me" {
///             dialog.accept(None)
///         } else {
///             dialog.dismiss()
///         };
///     }
///     _ => {}
///   }
/// }))?;
/// #
/// # Ok(())
/// # }
/// ```
pub struct Dialog {
    session_id: SessionId,
    transport: Arc<Transport>,
}

impl Dialog {
    pub(crate) fn new(session_id: SessionId, transport: Arc<Transport>) -> Self {
        Self {
            session_id,
            transport,
        }
    }

    fn handle(&self, accept: bool, prompt_text: Option<String>) -> Result<()> {
        self.transport.call_method_on_target(
            self.session_id.clone(),
            HandleJavaScriptDialog {
                accept,
                prompt_text,
            },
        )?;

        Ok(())
    }

    /// Press the `OK` button on the dialog. You can specify a prompt value with `prompt_text`.
    pub fn accept(&self, prompt_text: Option<String>) -> Result<()> {
        self.handle(true, prompt_text)
    }

    /// Press the `Cancel` button on the dialog or close it.
    pub fn dismiss(&self) -> Result<()> {
        self.handle(false, None)
    }
}
