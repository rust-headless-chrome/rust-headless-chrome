use std::borrow::Cow;
use std::sync::mpsc;

use cdp::{HasCdpCommand, SerializeCdpCommand};
use cdp::target::*;
use log::*;
use serde;
use serde::de::DeserializeOwned;
use serde_json::json;

use super::chrome;
use super::connection;
use super::errors::*;
use super::waiting_call_registry;
use super::waiting_call_registry::CallId;

pub struct PageSession {
    session_id: String,
    connection: connection::Connection,
    call_registry: waiting_call_registry::WaitingCallRegistry,
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
            target_id,
            flatten: Some(false),
        })?;
        let session_id = response.session_id.to_string();

        let call_registry = waiting_call_registry::WaitingCallRegistry::new(messages_rx);

        Ok(PageSession { session_id, connection: conn, call_registry })
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

        let _response: SendMessageToTargetResponse = self.connection.call_method(&session_command).expect("Didn't get confirmation from conn that our method was received...");

        // TODO: DRY up this and call_method in connection


        let response_rx = self.call_registry.register_call(call_id);

        trace!("waiting for response!");
        let raw_response = response_rx.recv().unwrap();
        trace!("method caller got response");
        let method_response = serde_json::from_value::<R>(raw_response.result).unwrap();
        Ok(method_response as R)
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
        let _resp = session.call_method::<cdp::page::CaptureScreenshotResponse>(&comm).unwrap();
        dbg!(_resp);
        std::thread::sleep(std::time::Duration::from_millis(1000));

//        session.goto("https://example.com");
    }
}
