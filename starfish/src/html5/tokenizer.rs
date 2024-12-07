
use crate::shared::byte_stream::ByteStream;

pub struct Tokenizer<'tokens> {
    pub stream: &'tokens mut ByteStream,
}
