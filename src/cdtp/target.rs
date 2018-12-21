use serde::{Serialize, Deserialize};

// TODO: for when this works with IntelliJ
//pub use events::*;
//pub use methods::*;

pub mod events {
    use serde::{Deserialize};
    #[derive(Deserialize, Debug)]
    pub struct AttachedToTargetEvent {
        pub params: AttachedToTargetParams
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachedToTargetParams {
        pub session_id: String,
        pub target_info: super::TargetInfo,
        pub waiting_for_debugger: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct ReceivedMessageFromTargetEvent {
        pub params: ReceivedMessageFromTargetParams
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ReceivedMessageFromTargetParams {
        pub session_id: String,
        pub target_id: String,
        pub message: String,
    }
}

pub mod methods {
    use serde::{Serialize, Deserialize};
    use crate::cdtp::{Method};


    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CaptureScreenshot {
        format: String
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CaptureScreenshotReturnObject {
        data: String
    }
    impl Method for CaptureScreenshot {
        const NAME: &'static str = "Page.captureScreenshot";
        type ReturnObject = CaptureScreenshotReturnObject;
    }


    #[derive(Serialize)]
    pub struct CreateBrowserContext {}
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateBrowserContextReturnObject {
        pub browser_context_id: String,
    }
    impl Method for CreateBrowserContext {
        const NAME: &'static str = "Target.createBrowserContext";
        type ReturnObject = CreateBrowserContextReturnObject;
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateTarget {
        pub url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Frame width in DIP \\(headless chrome only\\)."]
        pub width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Frame height in DIP \\(headless chrome only\\)."]
        pub height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub browser_context_id: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_begin_frame_control: Option<bool>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateTargetReturnObject {
        pub target_id: String,
    }
    impl Method for CreateTarget {
        const NAME: &'static str = "Target.createTarget";
        type ReturnObject = CreateTargetReturnObject;
    }


    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachToTarget {
        pub target_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub flatten: Option<bool>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachToTargetReturnObject {
        pub session_id: String,
    }
    impl Method for AttachToTarget {
        const NAME: &'static str = "Target.attachToTarget";
        type ReturnObject = AttachToTargetReturnObject;
    }

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: String,
    #[serde(rename = "type")]
    pub target_type: String,
    // TODO: enum?
    pub title: String,
    pub url: String,
    pub attached: bool,
    pub opener_id: Option<String>,
    pub browser_context_id: Option<String>,
}
