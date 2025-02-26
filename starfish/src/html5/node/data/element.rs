use std::{
    collections::HashMap,
    fmt::{self, Debug, Formatter},
};

use crate::{
    html5::node::HTML_NAMESPACE,
    interface::{
        config::HasDocument,
        node::ElementDataType,
    },
    shared::{
        document::DocumentHandle,
        node::NodeId,
    },
};

#[derive(Debug)]
pub struct ClassListImpl {
    /// a map of classes applied to an HTML element.
    /// key = name, value = is_active
    /// the is_active is used to toggle a class (JavaScript API)
    class_map: HashMap<String, bool>,
}

impl Clone for ClassListImpl {
    fn clone(&self) -> Self {
        Self {
            class_map: self.class_map.clone(),
        }
    }
}

impl PartialEq for ClassListImpl {
    fn eq(&self, other: &Self) -> bool {
        self.class_map == other.class_map
    }
}

impl Default for ClassListImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassListImpl {
    /// Initialise a new (empty) ClassList
    #[must_use]
    pub fn new() -> Self {
        Self {
            class_map: HashMap::new(),
        }
    }
}

impl From<&str> for ClassListImpl {
    fn from(class_string: &str) -> Self {
        let class_map_local = class_string
            .split_whitespace()
            .map(|class| (class.to_owned(), true))
            .collect::<HashMap<String, bool>>();

        ClassListImpl {
            class_map: class_map_local,
        }
    }
}

/// Data structure for element nodes
#[derive(PartialEq, Clone)]
pub struct ElementData<C: HasDocument> {
    pub doc_handle: DocumentHandle<C>,
    pub node_id: Option<NodeId>,
    pub name: String,
    pub namespace: Option<String>,
    pub attributes: HashMap<String, String>,
    pub class_list: ClassListImpl,
    pub force_async: bool,
    pub template_contents: Option<C::DocumentFragment>,
}

impl<C: HasDocument> Debug for ElementData<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("ElementData");
        debug.field("name", &self.name);
        debug.field("attributes", &self.attributes);
        debug.field("classes", &self.class_list);
        debug.finish()
    }
}

impl<C: HasDocument> ElementDataType<C> for ElementData<C> {
    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn namespace(&self) -> &str {
        match self.namespace {
            Some(ref namespace) => namespace.as_str(),
            None => HTML_NAMESPACE,
        }
    }
}

impl<C: HasDocument> ElementData<C> {
    pub(crate) fn new(
        doc_handle: DocumentHandle<C>,
        name: &str,
        namespace: Option<&str>,
        attributes: HashMap<String, String>,
        classlist: ClassListImpl,
    ) -> Self {
        let (force_async, template_contents) = <_>::default();
        Self {
            doc_handle: doc_handle.clone(),
            node_id: None, // We are not yet registered in the document, so we have no node-id
            name: name.into(),
            namespace: Some(namespace.unwrap_or(HTML_NAMESPACE).into()),
            attributes,
            class_list: classlist,
            force_async,
            template_contents,
        }
    }
}
