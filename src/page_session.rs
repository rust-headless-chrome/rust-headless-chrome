use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use log::*;
use serde;
use error_chain::bail;

use crate::cdtp;
use crate::cdtp::{Message, Response, Event};
use crate::cdtp::target;
use crate::cdtp::input;
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
    pub navigating: Arc<Mutex<bool>>,
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

    // TODO: duplication between here and connection.call_method
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

        if let Some(error) = response.error {
            bail!(format!("{:?}", error))
        }

        let result: C::ReturnObject = serde_json::from_value(response.result.unwrap()).unwrap();

        dbg!(&result);

        Ok(result)
    }

}

#[cfg(test)]
mod tests {
    use crate::cdtp::page;
    use crate::cdtp::dom;
    use crate::cdtp::input;
    use crate::point::Point;
    use crate::cdtp::page::methods::*;
    use crate::errors::*;

    fn do_test() -> Result<()> {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true)?;
        let tab = chrome.new_tab()?;
        tab.navigate_to("http://todomvc.com/examples/vanillajs/");
        let element = tab.find_element("input")?;
        element.click();
        tab.type_str("buy cereal");
        tab.press_key("Enter");
        // TODO: raise error if node_id = 0
        let todo_label = tab.find_element("li label")?;
        let children = todo_label.get_description()?.children.unwrap();
        let text = &children.first().unwrap().node_value;
        assert_eq!("buy cereal", text);
        Ok(())
    }

    fn handles_remote_errors() -> Result<()> {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true)?;
        let tab = chrome.new_tab()?;
        tab.navigate_to("http://todomvc.com/examples/vanillajs/");

        // 0 is never a good node ID, AFAICT
        let node_description = tab.describe_node(0);
        assert_eq!(true, node_description.is_err());

        let element = tab.find_element("a pretty terrible CSS selector");
        assert_eq!(true, element.is_err());
        Ok(())
    }

    #[test]
    fn session_methods() {
        handles_remote_errors().expect("worked");
        do_test().expect("worked");
    }
}
