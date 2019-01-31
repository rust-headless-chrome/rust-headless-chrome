pub mod methods {
    use serde::{Deserialize, Serialize};
    use crate::cdtp::Method;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchMouseEvent {
        #[serde(rename = "type")]
        pub event_type: String,
        pub x: f64,
        pub y: f64,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchMouseEventReturnObject {
    }
    impl Method for DispatchMouseEvent {
        const NAME: &'static str = "Input.dispatchMouseEvent";
        type ReturnObject = DispatchMouseEventReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchKeyEvent {
        #[serde(rename = "type")]
        pub event_type: String,
        pub key: String,
        pub text: String,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchKeyEventReturnObject {
    }
    impl Method for DispatchKeyEvent {
        const NAME: &'static str = "Input.dispatchKeyEvent";
        type ReturnObject = DispatchKeyEventReturnObject;
    }
}

