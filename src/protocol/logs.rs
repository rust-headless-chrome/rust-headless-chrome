pub mod events {
    use crate::protocol::runtime::methods::{RemoteObject, StackTrace};
    use crate::protocol::types::{JsFloat, JsInt};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub enum LogEntrySource {
        XML,
        JavaScript,
        Network,
        Storage,
        AppCache,
        Rendering,
        Security,
        Deprecation,
        Worker,
        Violation,
        Intervention,
        Recommendation,
        Other,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
    #[serde(rename_all = "lowercase")]
    pub enum LogEntryLevel {
        Verbose,
        Info,
        Warning,
        Error,
    }

    /// Log entry for a logs
    /// See https://chromedevtools.github.io/devtools-protocol/tot/Log#type-LogEntry
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct LogEntry {
        pub source: LogEntrySource,
        pub level: LogEntryLevel,
        pub text: String,
        /// Timestamp when this entry was added
        /// Number of milliseconds since epoch
        /// See https://chromedevtools.github.io/devtools-protocol/tot/Runtime#type-Timestamp
        pub timestamp: JsFloat,
        pub url: Option<String>,
        pub line_number: Option<JsInt>,
        pub stack_trace: Option<StackTrace>,
        pub network_request_id: Option<String>,
        pub worker_id: Option<String>,
        pub args: Option<Vec<RemoteObject>>,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct EntryAddedParams {
        pub entry: LogEntry,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct EntryAddedEvent {
        pub params: EntryAddedParams,
    }

    #[test]
    fn can_parse_entry_added_event() {
        use serde_json::json;

        let json_message = json!({
          "entry": {
            "source": "violation",
            "level": "verbose",
            "text": "Added non-passive event listener to a scroll-blocking 'touchstart' event. Consider marking event handler as 'passive' to make the page more responsive. See https://www.chromestatus.com/feature/5745543795965952",
            "timestamp": 1565379708388.756,
            "url": "https://chromedevtools.github.io/devtools-protocol/index-imports.html",
            "lineNumber": 8,
            "stackTrace": {
              "callFrames": [
                {
                  "functionName": "_add",
                  "scriptId": "228",
                  "url": "https://chromedevtools.github.io/devtools-protocol/index-imports.html",
                  "lineNumber": 8,
                  "columnNumber": 52933
                },
                {
                  "functionName": "ready",
                  "scriptId": "238",
                  "url": "https://chromedevtools.github.io/devtools-protocol/index-imports.html",
                  "lineNumber": 8,
                  "columnNumber": 69089
                },
                {
                  "functionName": "ready",
                  "scriptId": "239",
                  "url": "https://chromedevtools.github.io/devtools-protocol/index-imports.html",
                  "lineNumber": 8,
                  "columnNumber": 74944
                },
                {
                  "functionName": "connectedCallback",
                  "scriptId": "230",
                  "url": "https://chromedevtools.github.io/devtools-protocol/index-imports.html",
                  "lineNumber": 8,
                  "columnNumber": 59068
                },
              ]
            }
          }
        });

        let _log_entry = serde_json::from_value::<LogEntry>(json_message["entry"].clone()).unwrap();

        let json_message = json!({
          "entry": {
            "source": "deprecation",
            "level": "warning",
            "text": "HTML Imports is deprecated and will be removed in M80, around February 2020. Please use ES modules instead. See https://www.chromestatus.com/features/5144752345317376 and https://developers.google.com/web/updates/2019/07/web-components-time-to-upgrade for more details.",
            "timestamp": 1565379707939.473
          }
        });

        let _log_entry = serde_json::from_value::<LogEntry>(json_message["entry"].clone()).unwrap();
    }
}

pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::protocol::types::JsUInt;
    use crate::protocol::Method;

    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub enum ViolationSettingName {
        LongTask,
        LongLayout,
        BlockedEvent,
        BlockedParser,
        DiscouragedAPIUse,
        Handler,
        RecurringHandler,
    }

    /// See https://chromedevtools.github.io/devtools-protocol/tot/Log#type-ViolationSetting
    #[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
    #[serde(rename_all = "camelCase")]
    pub struct ViolationSetting {
        name: ViolationSettingName,
        threshold: JsUInt,
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Enable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnableReturnObject {}
    impl Method for Enable {
        const NAME: &'static str = "Log.enable";
        type ReturnObject = EnableReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Disable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DisableReturnObject {}
    impl Method for Disable {
        const NAME: &'static str = "Log.disable";
        type ReturnObject = DisableReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Clear {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClearReturnObject {}
    impl Method for Clear {
        const NAME: &'static str = "Log.clear";
        type ReturnObject = DisableReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StartViolationsReport {
        pub config: Vec<ViolationSetting>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StartViolationsReportReturnObject {}
    impl Method for StartViolationsReport {
        const NAME: &'static str = "Log.startViolationsReport";
        type ReturnObject = StartViolationsReportReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct StopViolationsReport {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StopViolationsReportReturnObject {}
    impl Method for StopViolationsReport {
        const NAME: &'static str = "Log.stopViolationsReport";
        type ReturnObject = StopViolationsReportReturnObject;
    }
}
