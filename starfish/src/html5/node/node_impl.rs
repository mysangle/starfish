use std::fmt::Debug;

use crate::{
    html5::{
        document::document_impl::DocumentImpl,
        node::data::{
            comment::CommentData,
            doctype::DocTypeData,
            document::DocumentData,
            element::ElementData,
            text::TextData,
        },
    },
    interface::{
        config::HasDocument,
        node::{Node, NodeType, QuirksMode},
    },
    shared::{
        byte_stream::Location,
        document::DocumentHandle,
        node::NodeId,
    },
};

/// Implementation of the NodeDataType trait
#[derive(Debug, Clone, PartialEq)]
pub enum NodeDataTypeInternal<C: HasDocument> {
    /// Represents a document
    Document(DocumentData),
    // Represents a doctype
    DocType(DocTypeData),
    /// Represents a text
    Text(TextData),
    /// Represents a comment
    Comment(CommentData),
    /// Represents an element
    Element(ElementData<C>),
}

/// Node structure that resembles a DOM node
pub struct NodeImpl<C: HasDocument> {
    /// ID of the node, 0 is always the root / document node
    pub id: NodeId,
    /// parent of the node, if any
    pub parent: Option<NodeId>,
    /// any children of the node
    pub children: Vec<NodeId>,
    /// actual data of the node
    pub data: NodeDataTypeInternal<C>,
    /// Handle to the document in which this node resides
    pub document: DocumentHandle<C>,
    // Returns true when the given node is registered into the document arena
    pub registered: bool,
    // Location of the node in the source code
    pub location: Location,
}

impl<C: HasDocument<Document = DocumentImpl<C>>> Node<C> for NodeImpl<C> {
    type ElementData = ElementData<C>;

    fn id(&self) -> NodeId {
        self.id
    }

    fn set_id(&mut self, id: NodeId) {
        self.id = id
    }

    fn set_registered(&mut self, registered: bool) {
        self.registered = registered;
    }

    fn is_registered(&self) -> bool {
        self.registered
    }

    fn children(&self) -> &[NodeId] {
        self.children.as_slice()
    }

    fn type_of(&self) -> NodeType {
        match self.data {
            NodeDataTypeInternal::Document(_) => NodeType::DocumentNode,
            NodeDataTypeInternal::DocType(_) => NodeType::DocTypeNode,
            NodeDataTypeInternal::Text(_) => NodeType::TextNode,
            NodeDataTypeInternal::Comment(_) => NodeType::CommentNode,
            NodeDataTypeInternal::Element(_) => NodeType::ElementNode,
        }
    }

    fn is_element_node(&self) -> bool {
        self.type_of() == NodeType::ElementNode
    }

    fn get_element_data(&self) -> Option<&Self::ElementData> {
        if let NodeDataTypeInternal::Element(data) = &self.data {
            return Some(data);
        }
        None
    }

    fn get_element_data_mut(&mut self) -> Option<&mut ElementData<C>> {
        if let NodeDataTypeInternal::Element(data) = &mut self.data {
            return Some(data);
        }
        None
    }

    fn insert(&mut self, node_id: NodeId, idx: usize) {
        self.children.insert(idx, node_id);
    }

    fn push(&mut self, node_id: NodeId) {
        self.children.push(node_id);
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
        debug.field("parent", &self.parent);
        debug.field("children", &self.children);
        // @todo: add element/doctype etc data
        debug.finish_non_exhaustive()
    }
}

impl<C: HasDocument> Clone for NodeImpl<C> {
    fn clone(&self) -> Self {
        NodeImpl {
            id: self.id,
            parent: self.parent,
            children: self.children.clone(),
            data: self.data.clone(),
            document: self.document.clone(),
            registered: self.registered,
            location: self.location,
        }
    }
}

impl<C: HasDocument> NodeImpl<C> {
    /// create a new `Node`
    #[must_use]
    pub fn new(document: DocumentHandle<C>, location: Location, data: &NodeDataTypeInternal<C>) -> Self {
        let (id, parent, children, registered) = <_>::default();

        Self {
            id,
            parent,
            children,
            data: data.clone(),
            document: document.clone(),
            registered,
            location,
        }
    }

    /// Create a new document node
    #[must_use]
    pub fn new_document(document: DocumentHandle<C>, location: Location, quirks_mode: QuirksMode) -> Self {
        Self::new(
            document,
            location,
            &NodeDataTypeInternal::Document(DocumentData::new(quirks_mode)),
        )
    }
}
