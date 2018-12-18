use serde::{Serialize, Deserialize};

trait Session {
    fn send(&mut self, serialized_method_call: impl Into<String>) -> String;
}

struct Page {}

// should it come back as Value? Or maybe even the right return object?
impl Session for Page {
    fn send(&mut self, serialized_method_call: impl Into<String>) -> String {
        // real version sends this over a websocket
        r#"{"data": "asdfasdfadsf"}"#.to_string()
    }
}

// thought: could params and return object be captured in an enum struct for unsent / returned
//          method calls?

// TODO: Serialize (I think I can derive it)
#[derive(Serialize)]
struct CaptureScreenshotParams {
    format: String
}

// TODO: Deserialize
#[derive(Deserialize)]
struct CaptureScreenshotReturnObject {
    data: String
}

// https://chromedevtools.github.io/devtools-protocol/tot/Page#method-captureScreenshot
// TODO: do I really need this vs. just params?
struct CaptureScreenshot {}

// TODO: what about methods with no ReturnObject?
trait Method {
    const NAME: &'static str;

    type Parameters;
    type ReturnObject;

    fn call(&self, mut session: impl Session, params: Self::Parameters) -> Self::ReturnObject;
}

// TODO: have a seperate domain trait, and use that to generate the full name?
// TODO: make this generic!
impl Method for CaptureScreenshot {
    const NAME: &'static str = "Page.captureScreenshot";
    type Parameters = CaptureScreenshotParams;
    type ReturnObject = CaptureScreenshotReturnObject;

    fn call(&self, mut session: impl Session, params: Self::Parameters) -> Self::ReturnObject {
        let raw_response = &session.send("");
        // TODO: deal with badly formatted JSON (unlikely seeing as it's coming from chrome)
        serde_json::from_str(raw_response).unwrap()
    }
}

#[derive(Serialize)]
struct MethodCall<T> {
    method_name: String,
    id: u64,
    params: T,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializing() {
        env_logger::try_init().unwrap_or(());

        let page = Page {};
        // TODO: do I really need this vs. just params?
        let method = CaptureScreenshot {};
        let response = method.call(page, CaptureScreenshotParams { format: "png".into() });
        assert_eq!("asdfasdfadsf", response.data);
    }
}
