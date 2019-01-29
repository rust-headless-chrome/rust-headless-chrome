use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use log::*;
use serde;

use crate::cdtp;
use crate::cdtp::{Message, Response};
use crate::cdtp::target;
use crate::chrome;
use crate::connection;
use crate::errors::*;
use crate::waiting_call_registry;

pub struct PageSession {
    session_id: String,
    connection: connection::Connection,
    call_registry: waiting_call_registry::WaitingCallRegistry,
}


impl PageSession {
    pub fn new(browser_id: &chrome::BrowserId) -> Result<Self> {
        let (messages_tx, messages_rx) = mpsc::channel();
        let mut conn = super::connection::Connection::new(&browser_id, messages_tx).unwrap();

        let browser_context_id = conn.call_method(target::methods::CreateBrowserContext {})?.browser_context_id;
        let create_target = target::methods::CreateTarget {
            url: "about:blank".to_string(),
            width: None,
            height: None,
            browser_context_id: Some(browser_context_id),
            enable_begin_frame_control: None,
        };
        let target_id = conn.call_method(create_target)?.target_id;
        let session_id = conn.call_method(target::methods::AttachToTarget { target_id, flatten: None })?.session_id;

        let (responses_tx, responses_rx) = mpsc::channel();

        std::thread::spawn(move || {
            info!("starting msg handling loop");
            Self::handle_incoming_messages(messages_rx, responses_tx);
            info!("quit loop msg handling loop");
        });

        let call_registry = waiting_call_registry::WaitingCallRegistry::new(responses_rx);

        Ok(PageSession { session_id, connection: conn, call_registry })
    }

    fn handle_incoming_messages(messages_rx: Receiver<Message>, responses_tx: Sender<Response>) {
        for message in messages_rx {
            match message {
                Message::Event(event) => {
                    trace!("PageSession received event: {:?}", event);
                },
                Message::Response(response) => {
                   responses_tx.send(response).unwrap();
                },
            }
        }
    }

    pub fn call<C>(&mut self, method: C) -> Result<C::ReturnObject>
        where C: cdtp::Method + serde::Serialize {
        let method_call = method.to_method_call();
        let message = serde_json::to_string(&method_call).unwrap();

        let target_method = target::methods::SendMessageToTarget {
            target_id: None,
            session_id: Some(self.session_id.clone()),
            message
        };

        self.connection.call_method(target_method).unwrap();

        let response_rx = self.call_registry.register_call(method_call.id);

        let response = response_rx.recv().unwrap();

        let result: C::ReturnObject = serde_json::from_value(response.result).unwrap();

        dbg!(&result);

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::cdtp::page;
    use crate::cdtp::page::methods::*;

    #[test]
    fn session_methods() {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true).unwrap();

        // TODO: test you can make two sessions from one chrome thing!
        // inspect headfully at first!

        let mut session = super::PageSession::new(&chrome.browser_id).unwrap();

        let enable = Enable {};
        let enable_result = session.call(enable).unwrap();
        dbg!(enable_result);

        let navigate = Navigate { url: "https://wikipedia.org".to_string() };
        let nav_result = session.call(navigate).unwrap();
        dbg!(nav_result);

        std::thread::sleep(std::time::Duration::from_millis(1000));
//        let capture_screenshot = CaptureScreenshot { format: "png".to_string() };
//        let image_data = session.call(capture_screenshot).unwrap().data;
//
//        dbg!(image_data);

//        let comm = cdp::page::EnableCommand {};
////        dbg!(super::PageSession::command_for_session(session.session_id, &comm).unwrap());
//        let resp = session.call_method::<cdp::page::EnableResponse>(&comm);
//        dbg!(resp);
        std::thread::sleep(std::time::Duration::from_millis(1000));

//        session.goto("https://example.com");
    }
}
