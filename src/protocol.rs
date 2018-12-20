use serde::Deserialize;
use serde_json::Value;

pub type CallId = u16;

// this stuff should be in its own module b/c reused by page_session...
#[derive(Debug)]
pub struct Event {
    // TODO: could keep static const list of event names for sanity checking...
    name: String,
    params: Value,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodResponse {
    // TODO: should alias call IDs everywhere
    pub call_id: CallId,
    pub result: Value,
}


#[derive(Debug)]
pub enum IncomingMessageKind {
    Event(Event),
    MethodResponse(MethodResponse),
}

// TODO: custom deserialize?!
// TODO: Message term overloaded in context of websockets?
pub enum IncomingMessage<T> {
    FromBrowser(T),
    FromTarget(T),
}

#[derive(Deserialize, Debug)]
struct Response {
    id: CallId,
    result: Value,
}

#[derive(Deserialize, Debug)]
struct MyEvent {
    // also fail if it's an unknown event name!
    method: String, // TODO: we'll want to refer to this internally as method name or some such
    // TODO: should be one of predefined Parameter types ... and also have 'method'
    // NOTE: params is the data attached to an event
    params: Value,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum MyMessage {
    Event(MyEvent),
    Response(Response),
}

#[cfg(test)]
mod tests {
    use serde_json::json;

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
            let message: super::MyMessage = serde_json::from_str(&msg_string).unwrap();
            dbg!(message);
        }
    }
}

pub fn parse_raw_message(raw_message: &str) -> IncomingMessage<IncomingMessageKind> {
    let message: MyMessage = serde_json::from_str(&raw_message).unwrap();
    match message {
        MyMessage::Event(event) => {
            if let Value::String(response_string) = &event.params["message"] {
                // TODO: DRY
                let target_response: Response = serde_json::from_str(&response_string).unwrap();
                dbg!(&target_response);
                IncomingMessage::FromTarget(IncomingMessageKind::MethodResponse(MethodResponse {
                    call_id: target_response.id,
                    result: target_response.result,
                }))
            } else {
                IncomingMessage::FromTarget(IncomingMessageKind::Event(Event {
                    name: event.method,
                    params: event.params,
                }))
                // TODO: it's an event from the target? not sure.
            }
        }
        MyMessage::Response(response) => {
            IncomingMessage::FromBrowser(IncomingMessageKind::MethodResponse(MethodResponse {
                call_id: response.id,
                result: response.result,
            }))
        }
    }
}