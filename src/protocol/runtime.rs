pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::protocol::types::{JsInt, ScriptId, UniqueDebuggerId};
    use crate::protocol::Method;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct PropertyPreview {
        pub name: String,
        #[serde(rename = "type")]
        pub object_type: String,
        pub value: Option<String>,
        pub value_preview: Option<Box<PropertyPreview>>,
        pub subtype: Option<String>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ObjectPreview {
        #[serde(rename = "type")]
        pub object_type: String,
        pub subtype: Option<String>,
        pub description: Option<String>,
        pub overflow: bool,
        pub properties: Vec<PropertyPreview>,
    }

    /// Object type
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-RemoteObject
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub enum RemoteObjectType {
        Object,
        Function,
        Undefined,
        String,
        Number,
        Boolean,
        Symbol,
        Bigint,
    }

    /// Object subtype hint. Specified for object type values only
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-RemoteObject
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub enum RemoteObjectSubtype {
        Array,
        Null,
        Node,
        RegExp,
        Date,
        Map,
        Set,
        WeakMap,
        WeakSet,
        Iterator,
        Generator,
        Error,
        Proxy,
        Promise,
        TypedArray,
        ArrayBuffer,
        DataView,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct RemoteObject {
        #[serde(rename = "type")]
        pub object_type: RemoteObjectType,
        pub subtype: Option<RemoteObjectSubtype>,
        pub description: Option<String>,
        pub class_name: Option<String>,
        /// TODO: When the subtype is an array the returned `value` is always `None`.
        /// You can find the first 100 elements of the array in the `preview`.
        pub value: Option<serde_json::Value>,
        pub unserializable_value: Option<String>,
        pub preview: Option<ObjectPreview>,
    }

    /// If debuggerId is set stack trace comes from another debugger and can be resolved there.
    /// This allows to track cross-debugger calls. See Runtime.StackTrace and Debugger.paused for usages.
    /// Experimental feature of DevTools
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-StackTraceId
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    pub struct StackTraceId {
        id: String,
        debugger_id: UniqueDebuggerId,
    }

    /// Call frames for assertions or error messages
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-StackTrace
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct StackTrace {
        description: Option<String>,
        call_frames: Vec<CallFrame>,
        // parent: Option<StackTrace>,
        /// Asynchronous JavaScript stack trace that preceded this stack, if available.
        /// Experimental feature of DevTools
        /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-StackTraceId
        parent_id: Option<StackTraceId>,
    }

    /// Stack entry for runtime errors and assertions
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-CallFrame
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct CallFrame {
        function_name: String,
        script_id: ScriptId,
        url: String,
        line_number: JsInt,
        column_number: JsInt,
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CallArgument {
        pub value: serde_json::Value,
    }

    #[derive(Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct CallFunctionOn<'a> {
        pub object_id: &'a str,
        pub function_declaration: &'a str,
        pub arguments: Vec<CallArgument>,
        pub return_by_value: bool,
        pub generate_preview: bool,
        pub silent: bool,
        pub await_promise: bool,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CallFunctionOnReturnObject {
        pub result: RemoteObject,
    }
    impl<'a> Method for CallFunctionOn<'a> {
        const NAME: &'static str = "Runtime.callFunctionOn";
        type ReturnObject = CallFunctionOnReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Evaluate<'a> {
        pub expression: &'a str,
        pub include_command_line_api: bool,
        pub silent: bool,
        pub return_by_value: bool,
        pub generate_preview: bool,
        pub user_gesture: bool,
        pub await_promise: bool,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EvaluateReturnObject {
        pub result: RemoteObject,
    }
    impl<'a> Method for Evaluate<'a> {
        const NAME: &'static str = "Runtime.evaluate";
        type ReturnObject = EvaluateReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Enable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnableReturnObject {}
    impl Method for Enable {
        const NAME: &'static str = "Runtime.enable";
        type ReturnObject = EnableReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Disable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DisableReturnObject {}
    impl Method for Disable {
        const NAME: &'static str = "Runtime.disable";
        type ReturnObject = DisableReturnObject;
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AddBinding {
        pub name: String,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AddBindingReturnObject {}

    impl Method for AddBinding {
        const NAME: &'static str = "Runtime.addBinding";
        type ReturnObject = AddBindingReturnObject;
    }
}

pub mod events {
    use super::methods::{RemoteObject, StackTrace};
    use crate::protocol::types::{JsInt, ScriptId};
    use serde::{Deserialize, Serialize};

    /// Issued when exception was thrown and unhandled
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#event-exceptionThrown
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ExceptionThrown {
        pub timestamp: f64,
        pub exception_details: ExceptionDetails,
    }

    /// Detailed information about exception (or error) that was thrown during script compilation or execution
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-ExceptionDetails
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ExceptionDetails {
        pub exception_id: JsInt,
        pub text: String,
        pub line_number: JsInt,
        pub column_number: JsInt,
        pub script_id: Option<ScriptId>,
        pub url: Option<String>,
        pub stack_trace: Option<StackTrace>,
        pub exception: Option<RemoteObject>,
        pub execution_context_id: Option<JsInt>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ExceptionThrownEvent {
        pub params: ExceptionThrown,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct BindingCalledEvent {
        pub params: BindingCalledEventParams,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct BindingCalledEventParams {
        pub name: String,
        pub payload: String,
        pub execution_context_id: Option<JsInt>,
    }

    #[test]
    fn can_parse_exception_thrown_event() {
        let message = r#"
          {
              "timestamp": 1566067104960.9648,
              "exceptionDetails": {
                "exceptionId": 14,
                "text": "Uncaught",
                "lineNumber": 13,
                "columnNumber": 14,
                "url": "http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a",
                "stackTrace": {
                  "callFrames": [
                    {
                      "functionName": "thatThrows",
                      "scriptId": "179",
                      "url": "http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a",
                      "lineNumber": 13,
                      "columnNumber": 14
                    },
                    {
                      "functionName": "",
                      "scriptId": "179",
                      "url": "http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a",
                      "lineNumber": 10,
                      "columnNumber": 6
                    }
                  ]
                },
                "exception": {
                  "type": "object",
                  "subtype": "error",
                  "className": "Error",
                  "description": "Error: Just an error thrown()\n    at thatThrows (http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:14:15)\n    at http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:11:7",
                  "objectId": "{\"injectedScriptId\":45,\"id\":1}",
                  "preview": {
                    "type": "object",
                    "subtype": "error",
                    "description": "Error: Just an error thrown()\n    at thatThrows (http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:14:15)\n    at http://localhost:63342/rust-headless-chrome/events_fixtures/events_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:11:7",
                    "overflow": false,
                    "properties": [
                      {
                        "name": "stack",
                        "type": "string",
                        "value": "Error: Just an error thrown()\n    at thatThrows (https_page.html?_ijt=mf0c1t5voa9ogu1jan2ubc781a:11:7"
                      },
                      {
                        "name": "message",
                        "type": "string",
                        "value": "Just an error thrown()"
                      }
                    ]
                  }
                },
                "executionContextId": 45
              }
            }
        "#;

        let _exception_thrown = serde_json::from_str::<ExceptionThrown>(message).unwrap();
    }
}
