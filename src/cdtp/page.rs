pub mod methods {
    use serde::{Deserialize, Serialize};
    use crate::cdtp::Method;

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CaptureScreenshot {
        pub format: String
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct CaptureScreenshotReturnObject {
        pub data: String
    }

    impl Method for CaptureScreenshot {
        const NAME: &'static str = "Page.captureScreenshot";
        type ReturnObject = CaptureScreenshotReturnObject;
    }
}
