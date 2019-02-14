use serde::Deserialize;

pub type NodeId = u16;

pub type NodeAttributes = Vec<String>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub node_id: NodeId,
    pub backend_node_id: NodeId,
    pub children: Option<Vec<Node>>,
    pub node_value: String,
    pub node_name: String,
    pub node_type: u8,
    pub attributes: Option<NodeAttributes>, // TODO: there's way more here: https://chromedevtools.github.io/devtools-protocol/tot/DOM#type-Node
}

pub mod methods {
    use crate::cdtp::Method;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct GetDocument {
        pub depth: Option<u8>,
        pub pierce: Option<bool>,
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
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backend_node_id: Option<super::NodeId>,
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

    #[derive(Serialize, Debug, Default)]
    #[serde(rename_all = "camelCase")]
    pub struct Focus {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backend_node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub object_id: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FocusReturnObject {}
    impl Method for Focus {
        const NAME: &'static str = "DOM.focus";
        type ReturnObject = FocusReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct SetFileInputFiles<'a> {
        pub files: &'a [&'a str],
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backend_node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub object_id: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SetFileInputFilesReturnObject {}
    impl<'a> Method for SetFileInputFiles<'a> {
        const NAME: &'static str = "DOM.setFileInputFiles";
        type ReturnObject = SetFileInputFilesReturnObject;
    }

    #[derive(Serialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct QuerySelector<'a> {
        pub node_id: super::NodeId,
        pub selector: &'a str,
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct QuerySelectorReturnObject {
        pub node_id: super::NodeId,
    }
    impl<'a> Method for QuerySelector<'a> {
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
    pub struct GetContentQuads<'a> {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub backend_node_id: Option<super::NodeId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub object_id: Option<&'a str>,
        // TODO: two more fields here
    }
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetContentQuadsReturnObject {
        // TODO: use fixed sized array, check whether integers
        pub quads: Vec<[f64; 8]>,
    }
    impl<'a> Method for GetContentQuads<'a> {
        const NAME: &'static str = "DOM.getContentQuads";
        type ReturnObject = GetContentQuadsReturnObject;
    }
}
