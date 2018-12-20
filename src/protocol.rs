use serde;
use serde::Deserialize;
use serde_json::Value;

pub type CallId = u16;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Response {
    #[serde(rename(deserialize = "id"))]
    pub call_id: CallId,
    pub result: Value,
}

#[derive(Deserialize, Debug)]
pub struct MyEvent<P> {
    // also fail if it's an unknown event name!
//    pub method: String,
    // TODO: we'll want to refer to this internally as method name or some such
    // NOTE: params is the data attached to an event. from user's perspective, we don't need to keep
    // that name, because it's not even in the public docs for the protocol
    pub params: P,
}

mod target {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct AttachedToTargetParams {
        #[serde(rename = "sessionId")]
        #[doc = "Identifier assigned to the session used to send/receive messages."]
        pub session_id: String,
        #[serde(rename = "targetInfo")]
        pub target_info: TargetInfo,
        #[serde(rename = "waitingForDebugger")]
        pub waiting_for_debugger: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct TargetInfo {
        #[serde(rename = "targetId")]
        pub target_id: String,
        #[serde(rename = "type")]
        pub target_type: String, // TODO: enum?
        #[serde(rename = "title")]
        pub title: String,
        #[serde(rename = "url")]
        pub url: String,
        #[serde(rename = "attached")]
        #[doc = "Whether the target has an attached client."]
        pub attached: bool,
        #[serde(rename = "openerId")]
        #[doc = "Opener target Id"]
        pub opener_id: Option<String>,
        #[serde(rename = "browserContextId")]
        #[doc = "<span class=\"stab unstable\">[Experimental]</span>"]
        pub browser_context_id: Option<String>,
    }
}


#[derive(Deserialize, Debug)]
#[serde(tag = "method")]
pub enum EventMessage {
    #[serde(rename = "Target.attachedToTarget")]
    AttachedToTargetEvent(MyEvent<target::AttachedToTargetParams>),
    UnknownEvent(MyEvent<Value>)
}

#[derive(Deserialize, Debug)]
pub struct Event {
    // also fail if it's an unknown event name!
    pub method: String,
    // TODO: we'll want to refer to this internally as method name or some such
    // TODO: should be one of predefined Parameter types ... and also have 'method'
    // NOTE: params is the data attached to an event
    pub params: Value,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Message {
    Event(Event),
    Response(Response),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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

        dbg!(&attached_to_target_json);

        let event: EventMessage = serde_json::from_value(attached_to_target_json).unwrap();
        match event {
            EventMessage::AttachedToTargetEvent(_) => {

            },
            _ => {
                panic!("bad news");
            }
        }
        dbg!(event);
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
            let message: super::Message = serde_json::from_str(&msg_string).unwrap();
            dbg!(message);
        }
    }
}

pub fn parse_raw_message(raw_message: &str) -> Message {
    serde_json::from_str(&raw_message).unwrap()
}