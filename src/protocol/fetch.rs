use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct HeaderEntry {
    pub name: String,
    pub value: String,
}

pub mod methods {
    use crate::protocol::network::ErrorReason;
    use crate::protocol::types::JsUInt;
    use crate::protocol::Method;
    use serde::{Deserialize, Serialize};

    /// Fetch.enable
    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestPattern<'a> {
        /// Wildcards ('*' -> zero or more, '?' -> exactly one) are allowed.
        /// Escape character is backslash. Omitting is equivalent to "*".
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url_pattern: Option<&'a str>,
        /// Resource type as it was perceived by the rendering engine.
        ///
        /// Allowed values:
        /// Document, Stylesheet, Image, Media, Font, Script, TextTrack, XHR, Fetch, EventSource, WebSocket, Manifest, SignedExchange, Ping, CSPViolationReport, Other
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource_type: Option<&'a str>,

        /// Stages of the interception to begin intercepting. Request will intercept before the
        /// request is sent. Response will intercept after the response is received.
        ///
        /// Allowed values:
        /// Request, HeadersReceived
        #[serde(skip_serializing_if = "Option::is_none")]
        pub request_stage: Option<&'a str>,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Enable<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub patterns: Option<&'a [RequestPattern<'a>]>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub handle_auth_requests: Option<bool>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnableReturnObject {}

    impl<'a> Method for Enable<'a> {
        const NAME: &'static str = "Fetch.enable";
        type ReturnObject = EnableReturnObject;
    }

    /// Fetch.disable
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Disable {}

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DisableReturnObject {}

    impl Method for Disable {
        const NAME: &'static str = "Fetch.disable";
        type ReturnObject = DisableReturnObject;
    }

    /// Fetch.failRequest
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FailRequest {
        pub request_id: String,
        pub error_reason: ErrorReason,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FailRequestReturnObject {}

    impl Method for FailRequest {
        const NAME: &'static str = "Fetch.failRequest";
        type ReturnObject = FailRequestReturnObject;
    }

    /// Fetch.fulfillRequest
    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FulfillRequest {
        pub request_id: String,
        pub response_code: JsUInt,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub response_headers: Option<Vec<super::HeaderEntry>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub binary_response_headers: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub body: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub response_phrase: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FulfillRequestReturnObject {}

    impl Method for FulfillRequest {
        const NAME: &'static str = "Fetch.fulfillRequest";
        type ReturnObject = FulfillRequestReturnObject;
    }

    /// Fetch.continueRequest
    #[derive(Debug, Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ContinueRequest {
        pub request_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub post_data: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<Vec<super::HeaderEntry>>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ContinueRequestReturnObject {}

    impl Method for ContinueRequest {
        const NAME: &'static str = "Fetch.continueRequest";
        type ReturnObject = ContinueRequestReturnObject;
    }

    /// Fetch.continueWithAuth
    #[derive(Serialize, Debug, Clone, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthChallengeResponse {
        /// Possible values: Default, CancelAuth, ProvideCredentials
        pub response: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<String>,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ContinueWithAuth<'a> {
        pub request_id: &'a str,
        pub auth_challenge_response: AuthChallengeResponse,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ContinueWithAuthReturnObject {}

    impl<'a> Method for ContinueWithAuth<'a> {
        const NAME: &'static str = "Fetch.continueWithAuth";
        type ReturnObject = ContinueWithAuthReturnObject;
    }

    /// Fetch.getResponseBody
    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetResponseBody<'a> {
        pub request_id: &'a str,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetResponseBodyReturnObject {
        pub body: String,
        pub base64_encoded: bool,
    }

    impl<'a> Method for GetResponseBody<'a> {
        const NAME: &'static str = "Fetch.getResponseBody";
        type ReturnObject = GetResponseBodyReturnObject;
    }
}

pub mod events {
    use crate::protocol::network::events::ResourceType;
    use crate::protocol::network::{ErrorReason, Request};
    use crate::protocol::types::JsUInt;
    use serde::{Deserialize, Serialize};

    /// Fetch.requestPaused
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestPausedEvent {
        pub params: RequestPausedEventParams,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestPausedEventParams {
        pub request_id: String,
        pub request: Request,
        pub frame_id: String,
        pub resource_type: ResourceType,
        pub response_error_reason: Option<ErrorReason>,
        pub response_status_code: Option<JsUInt>,
        pub response_headers: Option<Vec<super::HeaderEntry>>,
        pub network_id: Option<String>,
    }

    /// Fetch.authRequired
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthChallenge {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Source of the authentication challenge. Allowed values: Server, Proxy
        pub source: Option<String>,
        pub origin: String,
        pub scheme: String,
        pub realm: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthRequiredEvent {
        pub params: AuthRequiredEventParams,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthRequiredEventParams {
        pub request_id: String,
        pub request: Request,
        pub frame_id: String,
        pub resource_type: ResourceType,
        pub auth_challenge: AuthChallenge,
    }
}
