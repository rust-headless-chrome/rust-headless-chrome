use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use log::*;
use serde;

use crate::cdtp;
use crate::cdtp::{Message, Response, Event};
use crate::cdtp::target;
use crate::cdtp::page;
use crate::cdtp::page::methods::*;
use crate::chrome;
use crate::connection;
use crate::errors::*;
use crate::waiting_call_registry;

// TODO: could have a better name like ... tab?

pub struct PageSession {
    session_id: String,
    connection: connection::Connection,
    call_registry: waiting_call_registry::WaitingCallRegistry,
    main_frame_id: String,
    navigating: Arc<Mutex<bool>>,
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
        let session_id = conn.call_method(target::methods::AttachToTarget {
            target_id: target_id.clone(),
            flatten: None,
        })?.session_id;

        let (responses_tx, responses_rx) = mpsc::channel();

        let navigating = Arc::new(Mutex::new(false));
        let navigating_clone = Arc::clone(&navigating);

        std::thread::spawn(move || {
            info!("starting msg handling loop");
            Self::handle_incoming_messages(messages_rx, responses_tx, navigating_clone);
            info!("quit loop msg handling loop");
        });

        let call_registry = waiting_call_registry::WaitingCallRegistry::new(responses_rx);

        let mut session = PageSession {
            session_id,
            connection: conn,
            call_registry,
            main_frame_id: target_id,
            // NOTE: this might have to updated if we allow navigating as part of page creation
            navigating,
        };

        session.call(Enable {}).unwrap();
        session.call(SetLifecycleEventsEnabled { enabled: true }).unwrap();

        Ok(session)
    }

    fn handle_incoming_messages(messages_rx: Receiver<Message>, responses_tx: Sender<Response>, navigating: Arc<Mutex<bool>>) {
        for message in messages_rx {
            match message {
                Message::Event(event) => {
                    trace!("PageSession received event: {:?}", event);
                    match event {
                        Event::LifecycleEvent(lifecycle_event) => {
                            match lifecycle_event.params.name.as_ref() {
                                "networkAlmostIdle" => {
                                    let mut nav = navigating.lock().unwrap();
                                    *nav = false;
                                }
                                "init" => {
                                    let mut nav = navigating.lock().unwrap();
                                    *nav = true;
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
//                    events_tx.send(event).unwrap();
                }
                Message::Response(response) => {
                    responses_tx.send(response).unwrap();
                }
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
            message,
        };

        self.connection.call_method(target_method).unwrap();

        let response_rx = self.call_registry.register_call(method_call.id);

        let response = response_rx.recv().unwrap();

        let result: C::ReturnObject = serde_json::from_value(response.result).unwrap();

        dbg!(&result);

        Ok(result)
    }

    pub fn navigate_to(&mut self, url: &str) {}
}

#[cfg(test)]
mod tests {
    use crate::cdtp::page;
    use crate::cdtp::page::methods::*;

    #[test]
    fn session_methods() {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true).unwrap();

        let mut session = super::PageSession::new(&chrome.browser_id).unwrap();

        let get_frame_tree = GetFrameTree {};
        let frame_tree_result = session.call(get_frame_tree).unwrap();

        let navigate = Navigate { url: "https://wikipedia.org".to_string() };
        let nav_result = session.call(navigate).unwrap();

        println!("waiting to start navigating");
        // wait for navigating to go to true
        loop {
            if (*session.navigating.lock().unwrap()) {
                break;
            }
        }
        println!("started navigating");

        // wait for navigating to go to false
        loop {
            if (!*session.navigating.lock().unwrap()) {
                break;
            }
        }

        println!("done navigating");
        std::thread::sleep(std::time::Duration::from_millis(1000));

        // something like:
        // session.on_event(FrameStoppedLoading)
        // wait until we see a framestoppedloading event?

        // can we start 'listening' to copies of events just before sending navigate, and then iterate through those?

        // do we want a queue of incoming events, and 'wait for' means popping off one by one until we find the one we want?
        // but whatever if you want to wait on two different ones? you'd be discarding it.

//        let capture_screenshot = CaptureScreenshot { format: "png".to_string() };
//        let image_data = session.call(capture_screenshot).unwrap().data;
//
//        dbg!(image_data);

//        let comm = cdp::page::EnableCommand {};
////        dbg!(super::PageSession::command_for_session(session.session_id, &comm).unwrap());
//        let resp = session.call_method::<cdp::page::EnableResponse>(&comm);
//        dbg!(resp);

//        session.goto("https://example.com");
    }
}
