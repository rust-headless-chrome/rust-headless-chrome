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

// this is what we can get over the wire from chrome
struct Message {
    // TODO: do erros have different fields?

    id: Option<CallId>,
    result: Option<Value>, // TODO: should be one of predefined ReturnObject types
    // NOTE: params is the data attached to an event
    params: Option<Value>, // TODO: should be one of predefined Parameter types ... and also have 'method'
    method: Option<String>, // TODO: we'll want to refer to this internally as method name or some such
}

fn parse_call_id(response_id: Value) -> Option<CallId> {
    match response_id {
        Value::Number(num) => {
            // use unwrap or else panic here?
            let call_id = num.as_u64().expect("Call ID is a serde number but can't be made into a CallId") as CallId;
            Some(call_id)
        }
        Value::Null => {
            None
        }
        _ => {
            panic!("Weird response ID: not a number or null: {:?}", &response_id)
        }
    }
}

pub fn parse_raw_message(raw_message: &str) -> IncomingMessage<IncomingMessageKind> {
    let json_message: Value = serde_json::from_str(raw_message).unwrap();
    let browser_call_id: Option<CallId> = parse_call_id(json_message["id"].clone());

    match browser_call_id {
        Some(call_id) => {
            // TODO: gross
            IncomingMessage::FromBrowser(IncomingMessageKind::MethodResponse(MethodResponse {
                call_id,
                result: json_message["result"].clone(),
            }))
        }
        None => {
            let params = &json_message["params"];
            if let Value::String(response_string) = &params["message"] {
                // TODO: DRY
                let target_response: Value = serde_json::from_str(&response_string).unwrap();
                dbg!(&target_response);
                let target_call_id = parse_call_id(target_response["id"].clone());
                IncomingMessage::FromTarget(IncomingMessageKind::MethodResponse(MethodResponse {
                    call_id: target_call_id.expect("Response has message but not call id"),
                    result: target_response["result"].clone(),
                }))
            } else {
                IncomingMessage::FromTarget(IncomingMessageKind::Event(Event {
                    name: json_message["method"].to_string(),
                    params: json_message["params"].clone(),
                }))
                // TODO: it's an event from the target? not sure.
            }
        }
    }
}