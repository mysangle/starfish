
use crate::{
    html5::{
        node::HTML_NAMESPACE,
        parser::Html5Parser,
        tokenizer::token::Token,
    },
    interface::{config::HasDocument, document::Document},
    shared::node::NodeId,
};

impl<C: HasDocument> Html5Parser<'_, C> {
    pub fn insert_doctype_element(&mut self, token: &Token) {
        let node = self.create_node(token, HTML_NAMESPACE);
        self.document.get_mut().register_node_at(node, NodeId::root(), None);
    }

    pub fn insert_document_element(&mut self, token: &Token) {
        let node = self.create_node(token, HTML_NAMESPACE);
        let node_id = self.document.get_mut().register_node_at(node, NodeId::root(), None);

        self.open_elements.push(node_id);
    }
}
