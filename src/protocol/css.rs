use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ComputedStyleProperty {
    pub name: String,
    pub value: String,
}

pub mod methods {

    use crate::protocol::{types::JsUInt, Method};

    pub type NodeId = JsUInt;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetComputedStyleForNode {
        pub node_id: NodeId,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetComputedStyleForNodeReturnObject {
        pub computed_style: Vec<super::ComputedStyleProperty>,
    }

    impl<'a> Method for GetComputedStyleForNode {
        const NAME: &'static str = "CSS.getComputedStyleForNode";
        type ReturnObject = GetComputedStyleForNodeReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Enable {}

    #[derive(Debug, Deserialize)]
    pub struct EnableReturnObject {}

    impl Method for Enable {
        const NAME: &'static str = "CSS.enable";

        type ReturnObject = EnableReturnObject;
    }
}
