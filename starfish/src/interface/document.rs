use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::{
    interface::{
        config::HasDocument,
        node::{Node, QuirksMode},
    },
    shared::{
        byte_stream::Location,
        document::DocumentHandle,
        node::NodeId,
    },
};

use url::Url;

pub trait Document<C: HasDocument<Document = Self>>: Sized + Display + Debug + PartialEq + 'static {
    type Node: Node<C>;

    // Creates a new doc with an optional document root node
    #[allow(clippy::new_ret_no_self)]
    fn new(document_type: DocumentType, url: Option<Url>, root_node: Option<Self::Node>) -> DocumentHandle<C>;

    fn url(&self) -> Option<Url>;

    fn doctype(&self) -> DocumentType;
    fn node_by_id(&self, node_id: NodeId) -> Option<&Self::Node>;

    fn add_stylesheet(&mut self, stylesheet: C::Stylesheet);
    /// Return the root node of the document
    fn get_root(&self) -> &Self::Node;
    fn attach_node(&mut self, node_id: NodeId, parent_id: NodeId, position: Option<usize>);

    fn update_node(&mut self, node: Self::Node);

    /// Return number of nodes in the document
    fn node_count(&self) -> usize;

    /// Register a new node
    fn register_node(&mut self, node: Self::Node) -> NodeId;
    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId;

    fn new_document_node(handle: DocumentHandle<C>, quirks_mode: QuirksMode, location: Location) -> Self::Node;
    fn new_doctype_node(
        handle: DocumentHandle<C>,
        name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
        location: Location,
    ) -> Self::Node;
    fn new_comment_node(handle: DocumentHandle<C>, comment: &str, location: Location) -> Self::Node;
    fn new_text_node(handle: DocumentHandle<C>, value: &str, location: Location) -> Self::Node;
    fn new_element_node(
        handle: DocumentHandle<C>,
        name: &str,
        namespace: Option<&str>,
        attributes: HashMap<String, String>,
        location: Location,
    ) -> Self::Node;
}

pub trait DocumentFragment<C: HasDocument>: Sized + Clone + PartialEq {

}

/// Type of the given document
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum DocumentType {
    /// HTML document
    HTML,
    /// Iframe source document
    IframeSrcDoc,
}

pub trait DocumentBuilder<C: HasDocument> {
    fn new_document(url: Option<Url>) -> DocumentHandle<C>;
}
