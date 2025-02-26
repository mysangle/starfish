use std::fmt::Debug;

use crate::{
    interface::config::HasDocument,
    shared::node::NodeId,
};

/// Different types of nodes that all have their own data structures (NodeData)
#[derive(Debug, PartialEq)]
pub enum NodeType {
    DocumentNode,
    DocTypeNode,
    TextNode,
    CommentNode,
    ElementNode,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum QuirksMode {
    Quirks,
    LimitedQuirks,
    NoQuirks,
}

pub trait ElementDataType<C: HasDocument> {
    fn name(&self) -> &str;
    fn namespace(&self) -> &str;
}

pub trait Node<C: HasDocument>: Clone + Debug + PartialEq {
    type ElementData: ElementDataType<C>;

    fn id(&self) -> NodeId;
    fn set_id(&mut self, id: NodeId);
    fn set_registered(&mut self, registered: bool);
    fn is_registered(&self) -> bool;
    fn children(&self) -> &[NodeId];
    fn type_of(&self) -> NodeType;
    fn is_element_node(&self) -> bool;
    fn get_element_data(&self) -> Option<&Self::ElementData>;
    fn get_element_data_mut(&mut self) -> Option<&mut Self::ElementData>;
    fn insert(&mut self, node_id: NodeId, idx: usize);
    fn push(&mut self, node_id: NodeId);
}
