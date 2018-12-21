mod target {
    use serde::Deserialize;

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
        pub target_info: TargetInfo,
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
}
