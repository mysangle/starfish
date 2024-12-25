use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Formatter, Debug},
    rc::Rc,
};

use crate::interface::config::HasDocument;

pub struct DocumentHandle<C: HasDocument>(pub Rc<RefCell<C::Document>>);

impl<C: HasDocument> Debug for DocumentHandle<C>
where
    C::Document: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0.borrow())
    }
}

impl<C: HasDocument> PartialEq for DocumentHandle<C>
where
    C::Document: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().eq(&other.0.borrow())
    }
}

impl<C: HasDocument> Clone for DocumentHandle<C> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<C: HasDocument> DocumentHandle<C> {
    /// Create a new DocumentHandle from a document
    pub fn create(document: C::Document) -> Self {
        DocumentHandle(Rc::new(RefCell::new(document)))
    }

    /// Returns the document as referenced by the handle
    pub fn get(&self) -> Ref<C::Document> {
        self.0.borrow()
    }

    /// Returns a
    pub fn get_mut(&mut self) -> RefMut<C::Document> {
        self.0.borrow_mut()
    }
}
