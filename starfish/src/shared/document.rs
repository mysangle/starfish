use std::{
    cell::RefCell,
    fmt::{Formatter, Debug},
    rc::Rc,
};

use crate::shared::traits::config::HasDocument;

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
