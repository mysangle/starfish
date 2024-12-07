
use crate::{
    html5::tokenizer::Tokenizer,
    shared::{
        document::DocumentHandle,
        traits::config::HasDocument,
    },
};

pub struct Html5Parser<'tokens, C: HasDocument> {
    tokenizer: Tokenizer<'tokens>,
    context_doc: Option<DocumentHandle<C>>,
}

impl<C: HasDocument> crate::shared::traits::html5::Html5Parser<C> for Html5Parser<'_, C> {

}
