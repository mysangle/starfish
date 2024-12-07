use std::collections::HashMap;

use crate::shared::{
    node::NodeId,
    traits::config::HasDocument,
};

#[derive(Debug, Clone)]
pub struct NodeArena<C: HasDocument> {
    nodes: HashMap<NodeId, C::Node>,
    next_id: NodeId,
}

impl<C: HasDocument> PartialEq for NodeArena<C> {
    fn eq(&self, other: &Self) -> bool {
        if self.next_id != other.next_id {
            return false;
        }

        self.nodes == other.nodes
    }
}

impl<C: HasDocument> NodeArena<C> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            next_id: NodeId::default(),
        }
    }
    
    /// Gets the node with the given id
    pub fn node_ref(&self, node_id: NodeId) -> Option<&C::Node> {
        self.nodes.get(&node_id)
    }
}
