type Headers = HashMap<String, String>;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub url: String,
    pub url_fragment: Option<String>,
    pub method: String,
    pub headers: Headers,
    pub post_data: Option<String>,
    pub has_post_data: Option<bool>,
    pub mixed_content_type: Option<String>,
    /// Loading priority of a resource request.
    /// Allow values: VeryLow, Low, Medium, High, VeryHigh
    pub initial_priority: String,
    /// The referrer policy of the request, as defined in https://www.w3.org/TR/referrer-policy/
    /// Allow values: unsafe-url, no-referrer-when-downgrade, no-referrer, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin
    pub referrer_policy: String,
    pub is_link_preload: Option<bool>,
}

pub mod events {
    use serde::{Deserialize, Serialize};

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
    pub struct RequestInterceptedEventParams {
        pub interception_id: String,
        pub request: super::Request,
        pub frame_id: String,
        pub resource_type: String,
        pub is_navigation_request: bool,
        pub is_download: Option<bool>,
        pub redirect_url: Option<String>,
        pub auth_challenge: Option<AuthChallenge>,
        /// Network level fetch failure reason.
        /// Allow values:
        /// Failed, Aborted, TimedOut, AccessDenied, ConnectionClosed, ConnectionReset, ConnectionRefused, ConnectionAborted, ConnectionFailed, NameNotResolved, InternetDisconnected, AddressUnreachable, BlockedByClient, BlockedByResponse
        pub response_error_reason: Option<String>,
        pub response_status_code: Option<i32>,
        pub response_headers: Option<super::Headers>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestInterceptedEvent {
        pub params: RequestInterceptedEventParams,
    }

    #[test]
    fn can_parse_request_intercepted_event() {
        use crate::protocol;
        use serde_json::json;

        let json_message = json!({
             "method":"Network.requestIntercepted",
             "params":{
                 "frameId":"41AF9B7E70803C38860A845DBEB8F85F",
                 "interceptionId":"id-1",
                 "isNavigationRequest":true,
                 "request":{
                     "headers":{
                         "Accept":"text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8",
                         "Upgrade-Insecure-Requests":"1",
                         "User-Agent":"Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) HeadlessChrome/72.0.3626.119 Safari/537.36"
                     },
                     "initialPriority":"VeryHigh",
                     "method":"GET",
                     "referrerPolicy":"no-referrer-when-downgrade",
                     "url":"http://127.0.0.1:38157/"
                 },
                 "resourceType":"Document"
             }
        });

        let _request =
            serde_json::from_value::<super::Request>(json_message["params"]["request"].clone())
                .unwrap();
        let _event = serde_json::from_value::<protocol::Message>(json_message).unwrap();
    }
}

pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::protocol::Method;
    use std::collections::HashMap;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Enable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnableReturnObject {}
    impl Method for Enable {
        const NAME: &'static str = "Network.enable";
        type ReturnObject = EnableReturnObject;
    }

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
        pub interception_stage: Option<&'a str>,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SetRequestInterception<'a> {
        pub patterns: &'a [RequestPattern<'a>],
    }
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct SetRequestInterceptionReturnObject {}
    impl<'a> Method for SetRequestInterception<'a> {
        const NAME: &'static str = "Network.setRequestInterception";
        type ReturnObject = SetRequestInterceptionReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct AuthChallengeResponse<'a> {
        pub response: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub username: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub password: Option<&'a str>,
    }

    #[derive(Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct ContinueInterceptedRequest<'a> {
        pub interception_id: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub error_reason: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub raw_response: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub method: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub post_data: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub headers: Option<HashMap<&'a str, &'a str>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub auth_challenge_response: Option<AuthChallengeResponse<'a>>,
    }
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ContinueInterceptedRequestReturnObject {}
    impl<'a> Method for ContinueInterceptedRequest<'a> {
        const NAME: &'static str = "Network.continueInterceptedRequest";
        type ReturnObject = ContinueInterceptedRequestReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetResponseBodyForInterception<'a> {
        pub interception_id: &'a str,
    }
    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GetResponseBodyForInterceptionReturnObject {
        pub body: String,
        pub base64_encoded: bool,
    }
    impl<'a> Method for GetResponseBodyForInterception<'a> {
        const NAME: &'static str = "Network.getResponseBodyForInterception";
        type ReturnObject = GetResponseBodyForInterceptionReturnObject;
    }

}
