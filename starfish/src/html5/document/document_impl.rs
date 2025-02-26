use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Display, Formatter},
};

use crate::{
    html5::{
        document::task_queue::is_valid_id_attribute_value,
        node::{
            arena::NodeArena,
            data::{
                comment::CommentData,
                doctype::DocTypeData,
                document::DocumentData,
                element::{ClassListImpl, ElementData},
                text::TextData,
            },
            node_impl::{NodeDataTypeInternal, NodeImpl},
            HTML_NAMESPACE,
        },
    },
    interface::{
        config::HasDocument,
        document::{Document, DocumentType},
        node::{Node, QuirksMode},
    },
    shared::{
        byte_stream::Location,
        document::DocumentHandle,
        node::NodeId,
    },
};

use url::Url;

#[derive(Debug)]
pub struct DocumentImpl<C: HasDocument> {
    pub url: Option<Url>,
    pub(crate) arena: NodeArena<C>,
    named_id_elements: HashMap<String, NodeId>,
    pub doctype: DocumentType,
    pub quirks_mode: QuirksMode,
    pub stylesheets: Vec<C::Stylesheet>,
}

impl<C: HasDocument> PartialEq for DocumentImpl<C> {
    fn eq(&self, other: &Self) -> bool {
        self.arena == other.arena
    }
}

impl<C: HasDocument<Document = Self>> Document<C> for DocumentImpl<C> {
    type Node = NodeImpl<C>;

    /// Creates a new document without a doc handle
    #[must_use]
    fn new(document_type: DocumentType, url: Option<Url>, root_node: Option<Self::Node>) -> DocumentHandle<C> {
        let mut doc = Self {
            url,
            arena: NodeArena::new(),
            named_id_elements: HashMap::new(),
            doctype: document_type,
            quirks_mode: QuirksMode::NoQuirks,
            stylesheets: Vec::new(),
        };

        if let Some(node) = root_node {
            doc.register_node(node);

            DocumentHandle::create(doc)
        } else {
            let mut doc_handle = DocumentHandle::create(doc);
            let node = Self::Node::new_document(doc_handle.clone(), Location::default(), QuirksMode::NoQuirks);
            doc_handle.get_mut().arena.register_node(node);

            doc_handle
        }
    }

    fn url(&self) -> Option<Url> {
        self.url.clone()
    }

    fn doctype(&self) -> DocumentType {
        self.doctype
    }

    fn node_by_id(&self, node_id: NodeId) -> Option<&Self::Node> {
        self.arena.node_ref(node_id)
    }

    fn add_stylesheet(&mut self, stylesheet: C::Stylesheet) {
        self.stylesheets.push(stylesheet);
    }

    fn get_root(&self) -> &Self::Node {
        self.arena.node_ref(NodeId::root()).expect("Root node not found !?")
    }

    fn attach_node(&mut self, node_id: NodeId, parent_id: NodeId, position: Option<usize>) {
        // Check if any children of node have parent as child. This will keep adding the node to itself
        if parent_id == node_id || self.has_node_id_recursive(node_id, parent_id) {
            return;
        }

        if let Some(parent_node) = self.node_by_id(parent_id) {
            let mut parent_node = parent_node.clone();

            // Make sure position can never be larger than the number of children in the parent
            match position {
                Some(position) => {
                    if position > parent_node.children().len() {
                        parent_node.push(node_id);
                    } else {
                        parent_node.insert(node_id, position);
                    }
                }
                None => {
                    // No position given, add to end of the children list
                    parent_node.push(node_id);
                }
            }

            self.update_node(parent_node);
        }

        let mut node = self.arena.node(node_id).unwrap();
        node.parent = Some(parent_id);
        self.update_node(node);
    }

    fn update_node(&mut self, node: Self::Node) {
        if !node.is_registered() {
            tracing::warn!("Node is not registered to the arena");
            return;
        }

        self.on_document_node_mutation(&node);
        self.arena.update_node(node);
    }

    fn node_count(&self) -> usize {
        self.arena.node_count()
    }

    /// Register a node. It is not connected to anything yet, but it does receive a nodeId
    fn register_node(&mut self, mut node: Self::Node) -> NodeId {
        let node_id = self.arena.get_next_id();

        node.set_id(node_id);

        if node.is_element_node() {
            let element_data = node.get_element_data_mut().unwrap();
            element_data.node_id = Some(node_id);
        }

        self.on_document_node_mutation(&node);

        self.arena.register_node_with_node_id(node, node_id);

        node_id
    }

    fn register_node_at(&mut self, node: Self::Node, parent_id: NodeId, position: Option<usize>) -> NodeId {
        self.on_document_node_mutation(&node);

        let node_id = self.register_node(node);
        self.attach_node(node_id, parent_id, position);

        node_id
    }

    fn new_document_node(handle: DocumentHandle<C>, quirks_mode: QuirksMode, location: Location) -> Self::Node {
        NodeImpl::new(
            handle.clone(),
            location,
            &NodeDataTypeInternal::Document(DocumentData::new(quirks_mode)),
        )
    }

    fn new_doctype_node(
        handle: DocumentHandle<C>,
        name: &str,
        public_id: Option<&str>,
        system_id: Option<&str>,
        location: Location,
    ) -> Self::Node {
        NodeImpl::new(
            handle.clone(),
            location,
            &NodeDataTypeInternal::DocType(DocTypeData::new(name, public_id.unwrap_or(""), system_id.unwrap_or(""))),
        )
    }

    /// Creates a new comment node
    fn new_comment_node(handle: DocumentHandle<C>, comment: &str, location: Location) -> Self::Node {
        NodeImpl::new(
            handle.clone(),
            location,
            &NodeDataTypeInternal::Comment(CommentData::with_value(comment)),
        )
    }

    /// Creates a new text node
    fn new_text_node(handle: DocumentHandle<C>, value: &str, location: Location) -> Self::Node {
        NodeImpl::new(
            handle.clone(),
            location,
            &NodeDataTypeInternal::Text(TextData::with_value(value)),
        )
    }

    /// Creates a new element node
    fn new_element_node(
        handle: DocumentHandle<C>,
        name: &str,
        namespace: Option<&str>,
        attributes: HashMap<String, String>,
        location: Location,
    ) -> Self::Node {
        // Extract class list from the class-attribute (if exists)
        let class_list = match attributes.get("class") {
            Some(class_value) => ClassListImpl::from(class_value.as_str()),
            None => ClassListImpl::default(),
        };

        NodeImpl::new(
            handle.clone(),
            location,
            &NodeDataTypeInternal::Element(ElementData::new(
                handle.clone(),
                name,
                namespace,
                attributes,
                class_list,
            )),
        )
    }
}

impl<C: HasDocument<Document = Self>> DocumentImpl<C> {
    // Called whenever a node is being mutated in the document.
    fn on_document_node_mutation(&mut self, node: &NodeImpl<C>) {
        // self.on_document_node_mutation_update_id_in_node(node);
        self.on_document_node_mutation_update_named_id(node);
    }

    /// Update document's named id structure when the node has ID elements
    fn on_document_node_mutation_update_named_id(&mut self, node: &NodeImpl<C>) {
        if !node.is_element_node() {
            return;
        }

        let element_data = node.get_element_data().unwrap();
        if let Some(id_value) = element_data.attributes.get("id") {
            // When we have an ID attribute: update the named ID element map.
            if is_valid_id_attribute_value(id_value) {
                match self.named_id_elements.entry(id_value.clone()) {
                    Entry::Vacant(entry) => {
                        entry.insert(node.id());
                    }
                    Entry::Occupied(_) => {}
                }
            }
        } else {
            // If we don't have an ID attribute in the node, make sure that we remove and "old" id's that might be in the map.
            self.named_id_elements.retain(|_, id| *id != node.id());
        }
    }

    pub fn print_tree(&self, node: &C::Node, prefix: String, last: bool, f: &mut Formatter) {
    }

    pub fn has_node_id_recursive(&self, parent_id: NodeId, target_node_id: NodeId) -> bool {
        let parent = self.arena.node_ref(parent_id);
        if parent.is_none() {
            return false;
        }

        for child_node_id in parent.unwrap().children() {
            if *child_node_id == target_node_id {
                return true;
            }
            if self.has_node_id_recursive(*child_node_id, target_node_id) {
                return true;
            }
        }

        false
    }
}

impl<C: HasDocument<Document = Self>> Display for DocumentImpl<C> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let root = self.get_root();
        self.print_tree(root, "".to_string(), true, f);
        Ok(())
    }
}
