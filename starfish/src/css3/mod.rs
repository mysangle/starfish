
use crate::{
    interface::{
        css3::CssOrigin,
        ParserConfig,
    },
    shared::{
        byte_stream::{ByteStream, Encoding, Location},
        errors::{CssError, CssResult},
    },
};

pub mod stylesheet;
pub mod system;
pub mod tokenizer;

use stylesheet::CssStylesheet;
use tokenizer::Tokenizer;

pub struct Css3<'stream> {
    /// The tokenizer is responsible for reading the input stream and
    pub tokenizer: Tokenizer<'stream>,
    /// When true, we allow values in argument lists.
    allow_values_in_argument_list: Vec<bool>,
    /// The parser configuration as given
    config: ParserConfig,
    /// Origin of the stream (useragent, inline etc.)
    origin: CssOrigin,
    /// Source of the stream (filename, url, etc.)
    source: String,
}

impl<'stream> Css3<'stream> {
    fn new(stream: &'stream mut ByteStream, config: ParserConfig, origin: CssOrigin, source: &str) -> Self {
        Self {
            tokenizer: Tokenizer::new(stream, Location::default()),
            allow_values_in_argument_list: Vec::new(),
            config,
            origin,
            source: source.to_string(),
        }
    }

    /// Parses a direct string to a CssStyleSheet
    pub fn parse_str(
        data: &str,
        config: ParserConfig,
        origin: CssOrigin,
        source_url: &str,
    ) -> CssResult<CssStylesheet> {
        let mut stream = ByteStream::new(Encoding::UTF8, None);
        stream.read_from_str(data, Some(Encoding::UTF8));
        stream.close();

        Css3::parse_stream(&mut stream, config, origin, source_url)
    }

    // Parses a direct stream to a CssStyleSheet
    pub fn parse_stream(
        stream: &mut ByteStream,
        config: ParserConfig,
        origin: CssOrigin,
        source_url: &str,
    ) -> CssResult<CssStylesheet> {
        Css3::new(stream, config, origin, source_url).parse()
    }

    fn parse(&mut self) -> CssResult<CssStylesheet> {
        Err(CssError::new("No node tree found"))
    }
}

// Loads the default user agent stylesheet
pub fn load_default_useragent_stylesheet() -> CssStylesheet {
    let url = "gosub:useragent.css";

    let config = ParserConfig {
        ignore_errors: true,
        match_values: true,
        ..Default::default()
    };

    let css_data = include_str!("./resources/useragent.css");
    Css3::parse_str(css_data, config, CssOrigin::UserAgent, url).expect("Could not parse useragent stylesheet")
}
