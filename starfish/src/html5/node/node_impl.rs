use std::fmt::Debug;

use crate::{
    html5::document::document_impl::DocumentImpl,
    shared::{
        document::DocumentHandle,
        node::NodeId,
        traits::{
            config::HasDocument,
            node::Node,
        },
    },
};

pub struct NodeImpl<C: HasDocument> {
    pub id: NodeId,
    pub document: DocumentHandle<C>,
}

impl<C: HasDocument<Document = DocumentImpl<C>>> Node<C> for NodeImpl<C> {
    fn id(&self) -> NodeId {
        self.id
    }
}

impl<C: HasDocument<Document = DocumentImpl<C>>> PartialEq for NodeImpl<C> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id()
    }
}

impl<C: HasDocument> Debug for NodeImpl<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug = f.debug_struct("Node");
        debug.field("id", &self.id);
        //debug.field("parent", &self.parent);
        //debug.field("children", &self.children);
        // @todo: add element/doctype etc data
        debug.finish_non_exhaustive()
    }
}

impl<C: HasDocument> Clone for NodeImpl<C> {
    fn clone(&self) -> Self {
        NodeImpl {
            id: self.id,
            //parent: self.parent,
            //children: self.children.clone(),
            //data: self.data.clone(),
            document: self.document.clone(),
            //registered: self.registered,
            //location: self.location,
        }
    }
}
