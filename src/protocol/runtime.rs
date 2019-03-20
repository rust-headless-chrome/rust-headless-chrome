pub mod methods {
    use crate::protocol::Method;
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PropertyPreview {
        pub name: String,
        #[serde(rename = "type")]
        pub object_type: String,
        pub value: Option<String>,
        pub value_preview: Option<Box<PropertyPreview>>,
        pub subtype: Option<String>,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ObjectPreview {
        #[serde(rename = "type")]
        pub object_type: String,
        pub subtype: Option<String>,
        pub description: Option<String>,
        pub overflow: bool,
        pub properties: Vec<PropertyPreview>,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RemoteObject {
        #[serde(rename = "type")]
        pub object_type: String,
        pub subtype: Option<String>,
        pub description: Option<String>,
        pub class_name: Option<String>,
        pub value: Option<serde_json::Value>,
        pub unserializable_value: Option<String>,
        pub preview: Option<ObjectPreview>,
    }

    #[derive(Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct CallFunctionOn<'a> {
        pub object_id: &'a str,
        pub function_declaration: &'a str,
        pub return_by_value: bool,
        pub generate_preview: bool,
        pub silent: bool,
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
}
