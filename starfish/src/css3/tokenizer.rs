
use crate::shared::byte_stream::{ByteStream, Location};

#[allow(dead_code)]
pub struct Tokenizer<'stream> {
    stream: &'stream mut ByteStream,
}

impl<'stream> Tokenizer<'stream> {
    pub fn new(stream: &'stream mut ByteStream, start_location: Location) -> Self {
        Self {
            stream,
        }
    }
}
