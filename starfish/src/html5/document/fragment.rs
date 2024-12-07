
use crate::shared::{
    document::DocumentHandle,
    traits::{config::HasDocument, document::DocumentFragment},
};

#[derive(PartialEq)]
pub struct DocumentFragmentImpl<C: HasDocument> {
    pub handle: DocumentHandle<C>,
}

impl<C: HasDocument> Clone for DocumentFragmentImpl<C> {
    /// Clones the document fragment
    fn clone(&self) -> Self {
        Self {
            //arena: self.arena.clone(),
            handle: self.handle.clone(),
            //host: self.host,
        }
    }
}

impl<C: HasDocument> DocumentFragment<C> for DocumentFragmentImpl<C> {

}
