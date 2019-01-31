use serde::{Deserialize};

type NodeId = u16;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_id: NodeId
    // TODO: there's way more here: https://chromedevtools.github.io/devtools-protocol/tot/DOM#type-Node
}

pub mod methods {
    use serde::{Deserialize, Serialize};
    use crate::cdtp::Method;

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetDocument {
        pub depth: Option<u8>,
        pub pierce: Option<bool>
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetDocumentReturnObject {
        pub root: super::Node,
    }
    impl Method for GetDocument {
        const NAME: &'static str = "DOM.getDocument";
        type ReturnObject = GetDocumentReturnObject;
    }


    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct QuerySelector {
        pub node_id: super::NodeId,
        pub selector: String
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct QuerySelectorReturnObject {
        pub node_id: super::NodeId,
    }
    impl Method for QuerySelector {
        const NAME: &'static str = "DOM.querySelector";
        type ReturnObject = QuerySelectorReturnObject;
    }


    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetContentQuads {
        pub node_id: Option<super::NodeId>,
        // TODO: two more fields here
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetContentQuadsReturnObject {
        // TODO: use fixed sized array, check whether integers
        pub quads: Vec<[f64; 8]>,
    }
    impl Method for GetContentQuads {
        const NAME: &'static str = "DOM.getContentQuads";
        type ReturnObject = GetContentQuadsReturnObject;
    }
}

