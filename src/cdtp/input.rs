pub mod methods {
    use serde::{Deserialize, Serialize};
    use crate::cdtp::Method;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchMouseEvent<'a> {
        #[serde(rename = "type")]
        pub event_type: &'a str,
        pub x: f64,
        pub y: f64,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchMouseEventReturnObject {
    }
    impl<'a> Method for DispatchMouseEvent<'a> {
        const NAME: &'static str = "Input.dispatchMouseEvent";
        type ReturnObject = DispatchMouseEventReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchKeyEvent<'a> {
        #[serde(rename = "type")]
        pub event_type: &'a str,
        pub key: &'a str,
        pub text: &'a str,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchKeyEventReturnObject {
    }
    impl<'a> Method for DispatchKeyEvent<'a> {
        const NAME: &'static str = "Input.dispatchKeyEvent";
        type ReturnObject = DispatchKeyEventReturnObject;
    }
}

