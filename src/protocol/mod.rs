//! For (de)serializing method calls and events from the Chrome DevTools Protocol.

use std::fmt::Debug;

use crate::protocol::types::{JsInt, JsUInt};
use failure::{Fail, Fallible};
use serde;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod browser;
pub mod debugger;
pub mod dom;
pub mod input;
pub mod logs;
pub mod network;
pub mod page;
pub mod profiler;
pub mod runtime;
pub mod target;
pub mod types;

/// Browser window Id
type WindowId = JsUInt;

pub type CallId = JsUInt;

#[derive(Serialize, Debug)]
pub struct MethodCall<T>
where
    T: Debug,
{
    #[serde(rename = "method")]
    method_name: &'static str,
    pub id: CallId,
    params: T,
}

impl<T> MethodCall<T>
where
    T: Debug,
{
    pub fn get_params(&self) -> &T {
        &self.params
    }
}

pub trait Method: Debug {
    const NAME: &'static str;

    type ReturnObject: serde::de::DeserializeOwned + std::fmt::Debug; // have this = something?

    // TODO: Rust IntelliJ says that to_* method calls "usuall take self by reference"
    //       Maybe that'd be better here?
    fn to_method_call(self, call_id: CallId) -> MethodCall<Self>
    where
        Self: std::marker::Sized,
    {
        MethodCall {
            id: call_id,
            params: self,
            method_name: Self::NAME,
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone, Fail)]
#[fail(display = "Method call error {}: {}", code, message)]
pub struct RemoteError {
    pub code: JsInt,
    pub message: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Response {
    #[serde(rename(deserialize = "id"))]
    pub call_id: CallId,
    pub result: Option<Value>,
    pub error: Option<RemoteError>,
}

pub fn parse_response<T>(response: Response) -> Fallible<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    if let Some(error) = response.error {
        return Err(error.into());
    }

    let result: T = serde_json::from_value(response.result.unwrap()).unwrap();

    Ok(result)
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "method")]
#[allow(clippy::large_enum_variant)]
pub enum Event {
    #[serde(rename = "Target.attachedToTarget")]
    AttachedToTarget(target::events::AttachedToTargetEvent),
    #[serde(rename = "Target.receivedMessageFromTarget")]
    ReceivedMessageFromTarget(target::events::ReceivedMessageFromTargetEvent),
    #[serde(rename = "Target.targetInfoChanged")]
    TargetInfoChanged(target::events::TargetInfoChangedEvent),
    #[serde(rename = "Target.targetCreated")]
    TargetCreated(target::events::TargetCreatedEvent),
    #[serde(rename = "Target.targetDestroyed")]
    TargetDestroyed(target::events::TargetDestroyedEvent),
    #[serde(rename = "Page.frameStartedLoading")]
    FrameStartedLoading(page::events::FrameStartedLoadingEvent),
    #[serde(rename = "Page.frameNavigated")]
    FrameNavigated(page::events::FrameNavigatedEvent),
    #[serde(rename = "Page.frameStoppedLoading")]
    FrameStoppedLoading(page::events::FrameStoppedLoadingEvent),
    #[serde(rename = "Page.lifecycleEvent")]
    Lifecycle(page::events::LifecycleEvent),
    #[serde(rename = "Network.requestIntercepted")]
    RequestIntercepted(network::events::RequestInterceptedEvent),
    #[serde(rename = "Network.responseReceived")]
    ResponseReceived(network::events::ResponseReceivedEvent),
    #[serde(rename = "Log.entryAdded")]
    LogEntryAdded(logs::events::EntryAddedEvent),
    #[serde(rename = "Runtime.exceptionThrown")]
    RuntimeExceptionThrown(runtime::events::ExceptionThrownEvent),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Event(Event),
    Response(Response),
    ConnectionShutdown,
}

#[cfg(test)]
mod tests {
    use log::*;
    use serde_json::json;

    use super::*;

    #[test]
    fn pass_through_channel() {
        env_logger::try_init().unwrap_or(());

        let attached_to_target_json = json!({
            "method": "Target.attachedToTarget",
            "params": {
                "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
                "targetInfo": {
                    "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A",
                    "type": "page",
                    "title": "",
                    "url": "about:blank",
                    "attached": true,
                    "browserContextId": "946423F3D201EFA1A5FCF3462E340C15"
                },
                "waitingForDebugger": false
            }
        });

        let _event: Message = serde_json::from_value(attached_to_target_json).unwrap();
    }

    #[test]
    fn parse_event_fully() {
        env_logger::try_init().unwrap_or(());

        let attached_to_target_json = json!({
            "method": "Target.attachedToTarget",
            "params": {
                "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
                "targetInfo": {
                    "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A",
                    "type": "page",
                    "title": "",
                    "url": "about:blank",
                    "attached": true,
                    "browserContextId": "946423F3D201EFA1A5FCF3462E340C15"
                },
                "waitingForDebugger": false
            }
        });

        if let Ok(Event::AttachedToTarget(_)) = serde_json::from_value(attached_to_target_json) {
        } else {
            panic!("Failed to parse event properly");
        }

        let received_target_msg_event = json!({
            "method": "Target.receivedMessageFromTarget",
            "params": {
                "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
                "message": "{\"id\":43473,\"result\":{\"data\":\"kDEgAABII=\"}}",
                "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A"
            }
        });
        let event: Event = serde_json::from_value(received_target_msg_event).unwrap();
        match event {
            Event::ReceivedMessageFromTarget(ev) => {
                trace!("{:?}", ev);
            }
            _ => panic!("bad news"),
        }
    }

    #[test]
    fn easy_parse_messages() {
        env_logger::try_init().unwrap_or(());

        let example_message_strings = [
            // browser method response:
            "{\"id\":1,\"result\":{\"browserContextIds\":[\"C2652EACAAA12B41038F1F2137C57A6E\"]}}",
            "{\"id\":2,\"result\":{\"targetInfos\":[{\"targetId\":\"225A1B90036320AB4DB2E28F04AA6EE0\",\"type\":\"page\",\"title\":\"\",\"url\":\"about:blank\",\"attached\":false,\"browserContextId\":\"04FB807A65CFCA420C03E1134EB9214E\"}]}}",
            "{\"id\":3,\"result\":{}}",
            // browser event:
            "{\"method\":\"Target.attachedToTarget\",\"params\":{\"sessionId\":\"8BEF122ABAB0C43B5729585A537F424A\",\"targetInfo\":{\"targetId\":\"26DEBCB2A45BEFC67A84012AC32C8B2A\",\"type\":\"page\",\"title\":\"\",\"url\":\"about:blank\",\"attached\":true,\"browserContextId\":\"946423F3D201EFA1A5FCF3462E340C15\"},\"waitingForDebugger\":false}}",
            // browser event which indicates target method response:
            "{\"method\":\"Target.receivedMessageFromTarget\",\"params\":{\"sessionId\":\"8BEF122ABAB0C43B5729585A537F424A\",\"message\":\"{\\\"id\\\":43473,\\\"result\\\":{\\\"data\\\":\\\"iVBORw0KGgoAAAANSUhEUgAAAyAAAAJYCAYAAACadoJwAAAMa0lEQVR4nO3XMQEAIAzAMMC/5+GiHCQK+nbPzCwAAIDAeR0AAAD8w4AAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABII=\\\"}}\",\"targetId\":\"26DEBCB2A45BEFC67A84012AC32C8B2A\"}}"
        ];

        for msg_string in &example_message_strings {
            let _message: super::Message = parse_raw_message(msg_string).unwrap();
        }
    }
}

pub fn parse_raw_message(raw_message: &str) -> Fallible<Message> {
    Ok(serde_json::from_str::<Message>(raw_message)?)
}
