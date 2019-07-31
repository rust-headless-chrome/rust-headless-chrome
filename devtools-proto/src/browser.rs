pub mod methods {
    use serde::{Deserialize, Serialize};

    use crate::Method;

    #[derive(Serialize, Debug)]
    pub struct GetVersion {}
    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    /// Version information returned by `Browser.getVersion`
    pub struct VersionInformationReturnObject {
        /// Protocol version
        pub protocol_version: String,
        /// Product version
        pub product: String,
        /// Product revision
        pub revision: String,
        /// User-Agent
        pub user_agent: String,
        /// V8 version.
        pub js_version: String,
    }
    impl Method for GetVersion {
        const NAME: &'static str = "Browser.getVersion";
        type ReturnObject = VersionInformationReturnObject;
    }
}
