use std::collections::HashMap;

use crate::{
    interface::{
        config::HasDocument,
        node::Node,
    },
    shared::node::NodeId,
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

    pub(crate) fn get_next_id(&mut self) -> NodeId {
        let node_id = self.next_id;
        self.next_id = node_id.next();

        node_id
    }
    
    /// Gets the node with the given id
    pub fn node_ref(&self, node_id: NodeId) -> Option<&C::Node> {
        self.nodes.get(&node_id)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn register_node_with_node_id(&mut self, mut node: C::Node, node_id: NodeId) {
        assert!(!node.is_registered(), "Node is already attached to an arena");

        node.set_id(node_id);
        node.set_registered(true);

        self.nodes.insert(node_id, node);
    }

    /// Registered an unregistered node into the arena
    pub fn register_node(&mut self, mut node: C::Node) -> NodeId {
        assert!(!node.is_registered(), "Node is already attached to an arena");

        let id = self.next_id;
        self.next_id = id.next();

        node.set_id(id);
        node.set_registered(true);

        self.nodes.insert(id, node);
        id
    }
}
