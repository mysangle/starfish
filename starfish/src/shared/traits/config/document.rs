use std::fmt::Debug;

use crate::shared::traits::{
    config::HasCssSystem,
    document::{Document, DocumentBuilder, DocumentFragment},
    html5::Html5Parser,
    node::Node,
};

pub trait HasDocument:
    Sized
    + Clone
    + Debug
    + PartialEq
    + HasCssSystem
    + 'static
    + HasDocumentExt<
        Self,
        Node = <Self::Document as Document<Self>>::Node,
    >
{
    type Document: Document<Self>;
    type DocumentFragment: DocumentFragment<Self>;

    type DocumentBuilder: DocumentBuilder<Self>;
}
pub trait HasHtmlParser: HasDocument {
    type HtmlParser: Html5Parser<Self>;
}

pub trait HasDocumentExt<C: HasDocument> {
    type Node: Node<C>;
}

impl<C: HasDocument> HasDocumentExt<C> for C {
    type Node = <C::Document as Document<Self>>::Node;
}
