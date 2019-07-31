pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::Method;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchMouseEvent<'a> {
        #[serde(rename = "type")]
        pub event_type: &'a str,
        pub x: f64,
        pub y: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub button: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub click_count: Option<u8>,
    }
    impl<'a> Default for DispatchMouseEvent<'a> {
        fn default() -> Self {
            DispatchMouseEvent {
                event_type: "mouseMoved",
                x: 0.0,
                y: 0.0,
                button: None,
                click_count: None,
            }
        }
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchMouseEventReturnObject {}
    impl<'a> Method for DispatchMouseEvent<'a> {
        const NAME: &'static str = "Input.dispatchMouseEvent";
        type ReturnObject = DispatchMouseEventReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchKeyEvent<'a> {
        #[serde(rename = "type")]
        pub event_type: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub key: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub text: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub code: Option<&'a str>,
        pub windows_virtual_key_code: u8,
        pub native_virtual_key_code: u8,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DispatchKeyEventReturnObject {}
    impl<'a> Method for DispatchKeyEvent<'a> {
        const NAME: &'static str = "Input.dispatchKeyEvent";
        type ReturnObject = DispatchKeyEventReturnObject;
    }
}
