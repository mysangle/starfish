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

/// Data structure for element nodes
#[derive(PartialEq, Clone)]
pub struct ElementData<C: HasDocument> {
    pub doc_handle: DocumentHandle<C>,
    pub node_id: Option<NodeId>,
    pub name: String,
    pub namespace: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl<C: HasDocument> Debug for ElementData<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("ElementData");
        debug.field("name", &self.name);
        debug.field("attributes", &self.attributes);
        //debug.field("classes", &self.class_list);
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
