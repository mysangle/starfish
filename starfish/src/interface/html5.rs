
use crate::{
    interface::config::HasDocument,
    shared::{
        byte_stream::ByteStream,
        document::DocumentHandle,
        types::{ParseError, Result},
    },
};

pub trait Html5Parser<C: HasDocument> {
    type Options: ParserOptions;

    fn parse(stream: &mut ByteStream, doc: DocumentHandle<C>, opts: Option<Self::Options>) -> Result<Vec<ParseError>>;
}

pub trait ParserOptions {
    fn new(scripting: bool) -> Self;
}
