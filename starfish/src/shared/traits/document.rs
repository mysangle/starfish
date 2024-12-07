use std::fmt::{Debug, Display};

use crate::shared::traits::{config::HasDocument, node::Node};

pub trait Document<C: HasDocument<Document = Self>>: Sized + Display + Debug + PartialEq + 'static {
    type Node: Node<C>;

    /// Return the root node of the document
    fn get_root(&self) -> &Self::Node;
}

pub trait DocumentFragment<C: HasDocument>: Sized + Clone + PartialEq {

}

pub trait DocumentBuilder<C: HasDocument> {

}
