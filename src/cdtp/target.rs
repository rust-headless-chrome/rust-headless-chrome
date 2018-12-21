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
    pub struct AttachedToTargetParams {
        #[serde(rename = "sessionId")]
        #[doc = "Identifier assigned to the session used to send/receive messages."]
        pub session_id: String,
        #[serde(rename = "targetInfo")]
        pub target_info: super::TargetInfo,
        #[serde(rename = "waitingForDebugger")]
        pub waiting_for_debugger: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct ReceivedMessageFromTargetEvent {
        pub params: ReceivedMessageFromTargetParams
    }

    #[derive(Deserialize, Debug)]
    pub struct ReceivedMessageFromTargetParams {
        #[serde(rename = "sessionId")]
        #[doc = "Identifier assigned to the session used to send/receive messages."]
        pub session_id: String,
        #[serde(rename = "targetId")]
        pub target_id: String,
        pub message: String,
    }
}

pub mod methods {
    use serde::{Serialize, Deserialize};
    use crate::cdtp::{Method};


    #[derive(Serialize)]
    pub struct CaptureScreenshot {
        format: String
    }
    #[derive(Debug, Deserialize)]
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
    pub struct CreateBrowserContextReturnObject {
        #[serde(rename = "browserContextId")]
        #[doc = "The id of the context created."]
        pub browser_context_id: String,
    }
    impl Method for CreateBrowserContext {
        const NAME: &'static str = "Target.createBrowserContext";
        type ReturnObject = CreateBrowserContextReturnObject;
    }

    #[derive(Serialize)]
    pub struct CreateTarget {
        #[serde(rename = "url")]
        #[doc = "The initial URL the page will be navigated to."]
        pub url: String,
        #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
        #[doc = "Frame width in DIP \\(headless chrome only\\)."]
        pub width: Option<i32>,
        #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
        #[doc = "Frame height in DIP \\(headless chrome only\\)."]
        pub height: Option<i32>,
        #[serde(rename = "browserContextId", skip_serializing_if = "Option::is_none")]
        #[doc = "The browser context to create the page in."]
        pub browser_context_id: Option<String>,
        #[serde(rename = "enableBeginFrameControl", skip_serializing_if = "Option::is_none")]
        #[doc = "<span class=\"stab unstable\">[Experimental]</span> Whether BeginFrames for this target will be controlled via DevTools \\(headless chrome only,\nnot supported on MacOS yet, false by default\\)."]
        pub enable_begin_frame_control: Option<bool>,
    }
    #[derive(Deserialize, Debug)]
    pub struct CreateTargetReturnObject {
        #[serde(rename = "targetId")]
        pub target_id: String,
    }
    impl Method for CreateTarget {
        const NAME: &'static str = "Target.createTarget";
        type ReturnObject = CreateTargetReturnObject;
    }


    #[derive(Serialize)]
    pub struct AttachToTarget {
        #[serde(rename = "targetId")]
        pub target_id: String,
        #[serde(rename = "flatten", skip_serializing_if = "Option::is_none")]
        pub flatten: Option<bool>,
    }
    #[derive(Deserialize, Debug)]
    pub struct AttachToTargetReturnObject {
        #[serde(rename = "sessionId")]
        pub session_id: String,
    }
    impl Method for AttachToTarget {
        const NAME: &'static str = "Target.attachToTarget";
        type ReturnObject = AttachToTargetReturnObject;
    }

}

#[derive(Deserialize, Debug)]
pub struct TargetInfo {
    #[serde(rename = "targetId")]
    pub target_id: String,
    #[serde(rename = "type")]
    pub target_type: String,
    // TODO: enum?
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "attached")]
    #[doc = "Whether the target has an attached client."]
    pub attached: bool,
    #[serde(rename = "openerId")]
    #[doc = "Opener target Id"]
    pub opener_id: Option<String>,
    #[serde(rename = "browserContextId")]
    #[doc = "<span class=\"stab unstable\">[Experimental]</span>"]
    pub browser_context_id: Option<String>,
}
