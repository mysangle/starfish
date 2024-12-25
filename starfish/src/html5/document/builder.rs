
use crate::{
    interface::{
        config::HasDocument,
        document::{Document, DocumentBuilder, DocumentType},
    },
    shared::document::DocumentHandle,
};

use url::Url;

pub struct DocumentBuilderImpl {}

impl<C: HasDocument> DocumentBuilder<C> for DocumentBuilderImpl {
    fn new_document(url: Option<Url>) -> DocumentHandle<C> {
        C::Document::new(DocumentType::HTML, url, None)
    }
}
