pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::protocol::Method;
    use crate::protocol::types::{UniqueDebuggerId, ScriptId, JsInt};

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

    #[derive(Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct CallFunctionOn<'a> {
        pub object_id: &'a str,
        pub function_declaration: &'a str,
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
}
