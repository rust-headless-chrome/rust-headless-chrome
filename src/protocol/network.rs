use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::protocol::types::{JsFloat, JsInt, JsUInt};

type Headers = HashMap<String, String>;

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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub url: String,
    pub status: JsUInt,
    pub status_text: String,
    pub headers: Headers,
    pub headers_text: Option<String>,
    pub mime_type: String,
    pub request_headers: Option<Headers>,
    pub request_headers_text: Option<String>,
    pub connection_reused: bool,
    pub connection_id: JsInt,
    #[serde(rename = "remoteIPAddress")]
    pub remote_ip_address: Option<String>,
    pub remote_port: Option<JsUInt>,
    pub from_disk_cache: Option<bool>,
    pub from_service_worker: Option<bool>,
    pub from_prefetch_cache: Option<bool>,
    pub encoded_data_length: JsUInt,
    pub protocol: Option<String>,
    // pub timing: Option<ResourceTiming>,
    // pub security_state: SecurityState,
    // pub security_details: Option<SecurityDetails>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum CookieSameSite {
    Strict,
    Lax,
    Extended,
    None,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub expires: JsFloat,
    pub size: JsUInt,
    pub http_only: bool,
    pub secure: bool,
    pub session: bool,
    pub same_site: Option<CookieSameSite>,
}

pub mod events {
    use crate::protocol::types::{JsFloat, JsInt};
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
        pub response_status_code: Option<JsInt>,
        pub response_headers: Option<super::Headers>,
    }

    #[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ResourceType {
        Document,
        Stylesheet,
        Image,
        Media,
        Font,
        Script,
        TextTrack,
        XHR,
        Fetch,
        EventSource,
        WebSocket,
        Manifest,
        SignedExchange,
        Ping,
        CSPViolationReport,
        Other,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestInterceptedEvent {
        pub params: RequestInterceptedEventParams,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ResponseReceivedEventParams {
        pub request_id: String,
        pub loader_id: String,
        pub timestamp: JsFloat,
        #[serde(rename = "type")]
        pub _type: ResourceType,
        pub response: super::Response,
        pub frame_id: Option<String>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ResponseReceivedEvent {
        pub params: ResponseReceivedEventParams,
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
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::protocol::network::Cookie;
    use crate::protocol::Method;

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

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetResponseBody<'a> {
        pub request_id: &'a str,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GetResponseBodyReturnObject {
        pub body: String,
        pub base64_encoded: bool,
    }

    impl<'a> Method for GetResponseBody<'a> {
        const NAME: &'static str = "Network.getResponseBody";
        type ReturnObject = GetResponseBodyReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SetUserAgentOverride<'a, 'b, 'c> {
        pub user_agent: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub accept_language: Option<&'b str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub platform: Option<&'c str>,
    }

    #[derive(Deserialize, Debug)]
    pub struct SetUserAgentOverrideReturnObject {}

    impl<'a, 'b, 'c> Method for SetUserAgentOverride<'a, 'b, 'c> {
        const NAME: &'static str = "Network.setUserAgentOverride";
        type ReturnObject = SetUserAgentOverrideReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetCookies {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub urls: Option<Vec<String>>,
    }

    #[derive(Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GetCookiesReturnObject {
        pub cookies: Vec<Cookie>,
    }

    impl<'a> Method for GetCookies {
        const NAME: &'static str = "Network.getCookies";
        type ReturnObject = GetCookiesReturnObject;
    }
}
