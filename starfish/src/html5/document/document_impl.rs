use std::fmt::{self, Display, Formatter};

use crate::{
    html5::node::{arena::NodeArena, node_impl::NodeImpl},
    shared::{
        node::NodeId,
        traits::{config::HasDocument, document::Document},
    },
};

#[derive(Debug)]
pub struct DocumentImpl<C: HasDocument> {
    pub(crate) arena: NodeArena<C>,
}

impl<C: HasDocument> PartialEq for DocumentImpl<C> {
    fn eq(&self, other: &Self) -> bool {
        self.arena == other.arena
    }
}

impl<C: HasDocument<Document = Self>> Document<C> for DocumentImpl<C> {
    type Node = NodeImpl<C>;

    fn get_root(&self) -> &Self::Node {
        self.arena.node_ref(NodeId::root()).expect("Root node not found !?")
    }
}

impl<C: HasDocument<Document = Self>> DocumentImpl<C> {
    pub fn print_tree(&self, node: &C::Node, prefix: String, last: bool, f: &mut Formatter) {
    }
}

impl<C: HasDocument<Document = Self>> Display for DocumentImpl<C> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let root = self.get_root();
        self.print_tree(root, "".to_string(), true, f);
        Ok(())
    }
}
