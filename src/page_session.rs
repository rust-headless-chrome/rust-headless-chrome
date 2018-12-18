use std::borrow::Cow;
use std::sync::mpsc;

use log::*;

use rand::prelude::*;

use serde::de::DeserializeOwned;
use serde_json::json;

use super::errors::*;
use super::connection;
use super::chrome;

use cdp::{HasCdpCommand, SerializeCdpCommand, CdpCommand};
use cdp::target::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;

type MethodResponseTx = mpsc::Sender<connection::MethodResponse>;
// boo, seems chrome freaks out with u64
type CallId = u32;

pub struct PageSession {
    session_id: String,
    connection: connection::Connection,
    waiting_calls: Arc<Mutex<HashMap<CallId, MethodResponseTx>>>,
}


impl PageSession {
    pub fn new(browser_id: &chrome::BrowserId) -> Result<Self> {
        let (messages_tx, messages_rx) = mpsc::channel();
        let mut conn = super::connection::Connection::new(&browser_id, messages_tx).unwrap();

        let browser_context_id = conn.call_method::<CreateBrowserContextResponse>(&CreateBrowserContextCommand {})?.browser_context_id;
        let create_target_command = CreateTargetCommand {
            url: Cow::from("about:blank".to_string()),
            width: None,
            height: None,
            browser_context_id: Some(browser_context_id),
            enable_begin_frame_control: None,
        };
        let target_id = conn.call_method::<CreateTargetResponse>(&create_target_command)?.target_id;

        let response: AttachToTargetResponse = conn.call_method(&cdp::target::AttachToTargetCommand {
            target_id: target_id,
            flatten: Some(false),
        })?;
        let session_id = response.session_id.to_string();

        let waiting_calls = Arc::new(Mutex::new(HashMap::new()));
        // TODO: can I clone inside spawn?
        let other_waiting_calls = Arc::clone(&waiting_calls);

        let _event_handling_thread = std::thread::spawn(move || {
            info!("starting session's event handling loop");
            Self::handle_incoming_messages(&other_waiting_calls, messages_rx);
            info!("stopped session's event handling loop");
        });

        Ok(PageSession { session_id, connection: conn, waiting_calls })
    }

    fn handle_incoming_messages(waiting_calls: &Arc<Mutex<HashMap<CallId, MethodResponseTx>>>,
                                mut rx: mpsc::Receiver<super::connection::IncomingMessageKind>) {
        for msg in rx {
            trace!("I am a target, I just received this msg:\n {:?}", msg);
            match msg {
                connection::IncomingMessageKind::MethodResponse(response) => {
                    // TODO: obviously naming overlap between "response" and "message". result?
                    trace!("incoming handler trying to lock waiting_calls");
                    let mut waiting_calls_mut = waiting_calls.lock().unwrap();
                    trace!("incoming handler locked waiting_calls");
                    let waiting_call_tx: MethodResponseTx = waiting_calls_mut.remove(&response.id).unwrap();

                    waiting_call_tx.send(response);
                }
                connection::IncomingMessageKind::Event(event) => {
                    // TODO: will eventually need to handle multiple targets!
                    trace!("rcv'd event: {:?}", event);
//                    events_tx.send(event);
                }
            }
        }
    }

    pub fn command_for_session<C>(session_id: String, command: &C, call_id: CallId) -> Result<SendMessageToTargetCommand>
        where C: SerializeCdpCommand + serde::ser::Serialize {
        let method = json!({"method": command.command_name(), "id": call_id, "params": command});
        let message_str = serde_json::to_string(&method).unwrap();
        Ok(cdp::target::SendMessageToTargetCommand {
            message: Cow::from(message_str.to_string()),
            target_id: None,
            session_id: Some(Cow::from(session_id)),
        })
    }

    pub fn call_method<'a, R>(&mut self, command: &R::Command) -> Result<R>
        where R: DeserializeOwned + HasCdpCommand<'a>,
              <R as cdp::HasCdpCommand<'a>>::Command: serde::ser::Serialize + SerializeCdpCommand {
        // TODO: make part of the command perhaps.
        let call_id = rand::random::<CallId>();
        let session_command;
        {
            session_command = Self::command_for_session(self.session_id.clone(), command, call_id).unwrap();
        }

        let response: SendMessageToTargetResponse = self.connection.call_method(&session_command).expect("Didn't get confirmation from conn that our method was received...");

        // TODO: DRY up this and call_method in connection

        let (response_tx, response_rx) = mpsc::channel();

        let my_clone = Arc::clone(&self.waiting_calls);

        {
            let mut waiting_calls = my_clone.lock().unwrap();
            waiting_calls.insert(call_id, response_tx);
        }

        trace!("waiting for response!");
        let raw_response = response_rx.recv().unwrap();
        trace!("method caller got response");
        let method_response = serde_json::from_value::<R>(raw_response.result).unwrap();
        Ok(method_response as R)
//    eprintln!("{:#?}", _response);
//    let comm = cdp::page::NavigateCommand {
//        url: std::borrow::Cow::Borrowed("https://wikipedia.org"),
//        referrer: None,
//        transition_type: None,
//        frame_id: None
//    };
//    let method = json!({"method": comm.command_name(), "id":99999, "params": comm});
//    let message_str = serde_json::to_string(&method).unwrap();
//    trace!("sending message: {:#?}", &message_str);
//    let _response: cdp::target::SendMessageToTargetResponse = chrome.call_method(&cdp::target::SendMessageToTargetCommand {
//        message: std::borrow::Cow::Borrowed(&message_str),
//        target_id: None,
//        session_id: Some(session_id.clone()),
//    })?;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn session_methods() {
        env_logger::try_init().unwrap_or(());
        let chrome = super::chrome::Chrome::new(true).unwrap();

        // TODO: test you can make two sessions from one chrome thing!
        // inspect headfully at first!

        let mut session = super::PageSession::new(&chrome.browser_id).unwrap();

//        let comm = cdp::page::EnableCommand {};
////        dbg!(super::PageSession::command_for_session(session.session_id, &comm).unwrap());
//        let resp = session.call_method::<cdp::page::EnableResponse>(&comm);
//        dbg!(resp);
//        std::thread::sleep(std::time::Duration::from_millis(1000));

        let comm = cdp::page::CaptureScreenshotCommand {
            format: None,
            quality: None,
            clip: None,
            from_surface: None,
        };
//        dbg!(super::PageSession::command_for_session(session.session_id, &comm).unwrap());
        let resp = session.call_method::<cdp::page::CaptureScreenshotResponse>(&comm);
        dbg!(resp);
        std::thread::sleep(std::time::Duration::from_millis(1000));

//        session.goto("https://example.com");
    }
}
