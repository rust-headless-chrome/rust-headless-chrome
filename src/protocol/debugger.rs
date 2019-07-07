pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::protocol::Method;

    #[derive(Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct GetScriptSource<'a> {
        pub script_id: &'a str,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetScriptSourceReturnObject {
        pub script_source: String,
    }
    impl<'a> Method for GetScriptSource<'a> {
        const NAME: &'static str = "Debugger.getScriptSource";
        type ReturnObject = GetScriptSourceReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Enable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct EnableReturnObject {}
    impl Method for Enable {
        const NAME: &'static str = "Debugger.enable";
        type ReturnObject = EnableReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Disable {}
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DisableReturnObject {}
    impl Method for Disable {
        const NAME: &'static str = "Debugger.disable";
        type ReturnObject = DisableReturnObject;
    }
}
