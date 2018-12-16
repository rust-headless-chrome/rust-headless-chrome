use std::borrow::Cow;

use serde::de::DeserializeOwned;
use serde_json::json;

use super::errors::*;
use super::connection;
use super::chrome;

use cdp::{HasCdpCommand, SerializeCdpCommand, CdpCommand};
use cdp::target::*;

pub struct PageSession {
    session_id: String,
    connection: connection::Connection,
}


impl PageSession {
    pub fn new(browser_id: &chrome::BrowserId) -> Result<Self> {
        let mut conn = super::connection::Connection::new(&browser_id).unwrap();

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

        Ok(PageSession { session_id, connection: conn })
    }

    pub fn command_for_session<C>(session_id: String, command: &C) -> Result<SendMessageToTargetCommand>
        where C: SerializeCdpCommand + serde::ser::Serialize {
        let method = json!({"method": command.command_name(), "id":9999, "params": command});
        let message_str = serde_json::to_string(&method).unwrap();
        Ok(cdp::target::SendMessageToTargetCommand {
            message: Cow::from(message_str.to_string()),
            target_id: None,
            session_id: Some(Cow::from(session_id)),
        })
    }

    pub fn call_method<'a, R>(&mut self, command: &R::Command) -> SendMessageToTargetResponse
        where R: DeserializeOwned + HasCdpCommand<'a>,
              <R as cdp::HasCdpCommand<'a>>::Command: serde::ser::Serialize + SerializeCdpCommand {

        let session_command;
        {
            session_command = Self::command_for_session(self.session_id.clone(), command).unwrap();
        }

        let response: SendMessageToTargetResponse = self.connection.call_method(&session_command).unwrap();
        return response;
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

        let comm = cdp::page::EnableCommand {};
//        dbg!(super::PageSession::command_for_session(session.session_id, &comm).unwrap());
        let resp = session.call_method::<cdp::page::EnableResponse>(&comm);
        dbg!(resp);
        std::thread::sleep(std::time::Duration::from_millis(2000));

//        session.goto("https://example.com");
    }
}
