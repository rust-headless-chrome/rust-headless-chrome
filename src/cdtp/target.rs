use serde::Deserialize;

pub type TargetId = String;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TargetInfo {
    pub target_id: TargetId,
    #[serde(rename = "type")]
    pub target_type: String,
    // TODO: enum?
    pub title: String,
    pub url: String,
    pub attached: bool,
    pub opener_id: Option<String>,
    pub browser_context_id: Option<String>,
}

pub mod events {
    use serde::Deserialize;

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
        pub target_id: super::TargetId,
        pub message: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct TargetInfoChangedEvent {
        pub params: TargetInfoChangedParams
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetInfoChangedParams {
        pub target_info: super::TargetInfo,
    }


    #[derive(Deserialize, Debug)]
    pub struct TargetCreatedEvent {
        pub params: TargetCreatedParams
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetCreatedParams {
        pub target_info: super::TargetInfo,
    }

    #[derive(Deserialize, Debug)]
    pub struct TargetDestroyedEvent {
        pub params: TargetDestroyedParams
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct TargetDestroyedParams {
        pub target_id: super::TargetId,
    }
}

pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::cdtp::Method;

    #[derive(Serialize)]
    pub struct GetTargets {}
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetTargetsReturnObject {
        pub target_infos: Vec<super::TargetInfo>,
    }
    impl Method for GetTargets {
        const NAME: &'static str = "Target.getTargets";
        type ReturnObject = GetTargetsReturnObject;
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
    pub struct CreateTarget<'a> {
        pub url: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Frame width in DIP \\(headless chrome only\\)."]
        pub width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[doc = "Frame height in DIP \\(headless chrome only\\)."]
        pub height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub browser_context_id: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_begin_frame_control: Option<bool>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct CreateTargetReturnObject {
        pub target_id: super::TargetId,
    }
    impl<'a> Method for CreateTarget<'a> {
        const NAME: &'static str = "Target.createTarget";
        type ReturnObject = CreateTargetReturnObject;
    }


    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachToTarget<'a> {
        pub target_id: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub flatten: Option<bool>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachToTargetReturnObject {
        pub session_id: String,
    }
    impl<'a> Method for AttachToTarget<'a> {
        const NAME: &'static str = "Target.attachToTarget";
        type ReturnObject = AttachToTargetReturnObject;
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachToBrowserTarget {}
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AttachToBrowserTargetReturnObject {
        pub session_id: String,
    }
    impl Method for AttachToBrowserTarget {
        const NAME: &'static str = "Target.attachToBrowserTarget";
        type ReturnObject = AttachToBrowserTargetReturnObject;
    }


    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SetDiscoverTargets {
        pub discover: bool,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SetDiscoverTargetsReturnObject {
    }
    impl Method for SetDiscoverTargets {
        const NAME: &'static str = "Target.setDiscoverTargets";
        type ReturnObject = SetDiscoverTargetsReturnObject;
    }


    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SendMessageToTarget<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub target_id: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub session_id: Option<&'a str>,
        pub message: &'a str,
    }
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SendMessageToTargetReturnObject {}
    impl<'a> Method for SendMessageToTarget<'a> {
        const NAME: &'static str = "Target.sendMessageToTarget";
        type ReturnObject = SendMessageToTargetReturnObject;
    }
}

