use serde::{Deserialize};

pub type NodeId = u16;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_id: NodeId,
    pub backend_node_id: NodeId,
    pub children: Option<Vec<Node>>,
    pub node_value: String,
    pub node_name: String,
    pub node_type: u8,
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
    pub struct DescribeNode {
        pub node_id: super::NodeId,
        pub depth: Option<i8>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DescribeNodeReturnObject {
        pub node: super::Node,
    }
    impl Method for DescribeNode {
        const NAME: &'static str = "DOM.describeNode";
        type ReturnObject = DescribeNodeReturnObject;
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

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RemoteObject {
        pub object_id: Option<String>,
    }
    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct ResolveNode {
        pub backend_node_id: Option<super::NodeId>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ResolveNodeReturnObject {
        // TODO: use fixed sized array, check whether integers
        pub object: RemoteObject,
    }
    impl Method for ResolveNode {
        const NAME: &'static str = "DOM.resolveNode";
        type ReturnObject = ResolveNodeReturnObject;
    }


    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetContentQuads {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backend_node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub object_id: Option<String>,
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

