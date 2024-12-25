use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    html5::{
        node::HTML_NAMESPACE,
        parser::errors::{ErrorLogger, ParserError},
        tokenizer::token::Token,
    },
    shared::{
        byte_stream::{
            ByteStream,
            Character::{self, Ch, StreamEnd},
            Location,
            LocationHandler,
            Stream,
        },
        types::Result,
    },
};

pub mod state;
pub mod token;

use state::State;

/// Constants that are not directly captured as visible chars
pub const CHAR_NUL: char = '\u{0000}';
pub const CHAR_TAB: char = '\u{0009}';
pub const CHAR_LF: char = '\u{000A}';
pub const CHAR_CR: char = '\u{000D}';
pub const CHAR_FF: char = '\u{000C}';
pub const CHAR_SPACE: char = '\u{0020}';
pub const CHAR_REPLACEMENT: char = '\u{FFFD}';

macro_rules! to_lowercase {
    ($c:expr) => {
        $c.to_lowercase().next().unwrap()
    };
}

/// This struct is a gateway between the parser and the tokenizer. It holds data that can be needed
/// by the tokenizer in certain cases. See https://github.com/gosub-browser/gosub-engine/issues/230 for
/// more information and how we should refactor this properly.
pub struct ParserData {
    pub adjusted_node_namespace: String,
}

impl Default for ParserData {
    fn default() -> Self {
        Self {
            adjusted_node_namespace: HTML_NAMESPACE.to_string(),
        }
    }
}

/// Options that can be passed to the tokenizer. Mostly needed when dealing with tests.
pub struct Options {
    /// Sets the initial state of the tokenizer. Normally only needed when dealing with tests
    pub initial_state: State,
    /// Sets the last starting tag in the tokenizer. Normally only needed when dealing with tests
    pub last_start_tag: String,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            initial_state: State::Data,
            last_start_tag: String::new(),
        }
    }
}

pub struct Tokenizer<'tokens> {
    pub stream: &'tokens mut ByteStream,
    location_handler: LocationHandler,
    pub state: State,
    pub consumed: String,
    pub current_token: Option<Token>,
    pub token_queue: Vec<Token>,
    pub last_start_token: String,
    pub last_token_location: Location,
    pub last_char: Character,
    pub error_logger: Rc<RefCell<ErrorLogger>>,
}

impl<'stream> Tokenizer<'stream> {
    #[must_use]
    pub fn new(
        stream: &'stream mut ByteStream,
        opts: Option<Options>,
        error_logger: Rc<RefCell<ErrorLogger>>,
        start_location: Location,
    ) -> Self {
        Self {
            stream,
            location_handler: LocationHandler::new(start_location),
            state: opts.as_ref().map_or(State::Data, |o| o.initial_state),
            consumed: String::new(),
            current_token: None,
            token_queue: vec![],
            last_start_token: opts.map_or(String::new(), |o| o.last_start_tag),
            last_token_location: Location::default(),
            last_char: StreamEnd,
            error_logger,
        }
    }

    #[inline]
    pub(crate) fn get_location(&self) -> Location {
        self.location_handler.cur_location
    }

    pub fn next_token(&mut self, parser_data: ParserData) -> Result<Token> {
        self.consume_stream(parser_data)?;

        if self.token_queue.is_empty() {
            return Ok(Token::Eof {
                location: self.get_location(),
            });
        }

        Ok(self.token_queue.remove(0))
    }

    fn consume_stream(&mut self, parser_data: ParserData) -> Result<()> {
        loop {
            if !self.token_queue.is_empty() {
                return Ok(());
            }

            match self.state {
                State::Data => {
                    let loc = self.get_location();
                    let c = self.read_char();
                    match c {
                        Ch('&') => self.state = State::CharacterReferenceInData,
                        Ch('<') => {
                            self.state = {
                                self.last_token_location = loc;
                                State::TagOpen
                            }
                        }
                        Ch(CHAR_NUL) => {
                            self.consume(c.into());
                            self.parse_error(ParserError::UnexpectedNullCharacter, loc);
                        }
                        StreamEnd => self.emit_token(Token::Eof {
                            location: self.get_location(),
                        }),
                        _ => self.consume(c.into()),
                    }
                }
                State::TagOpen => {
                    let loc = self.get_location();
                    let c = self.read_char();
                    match c {
                        Ch('!') => self.state = State::MarkupDeclarationOpen,
                        Ch('/') => self.state = State::EndTagOpen,
                        Ch(ch) if ch.is_ascii_alphabetic() => {
                            self.current_token = Some(Token::StartTag {
                                name: String::new(),
                                is_self_closing: false,
                                attributes: HashMap::new(),
                                location: self.last_token_location,
                            });
                            self.stream_prev();
                            self.state = State::TagName;
                        }
                        Ch('?') => {
                            self.current_token = Some(Token::Comment {
                                comment: String::new(),
                                location: self.last_token_location,
                            });
                            self.parse_error(ParserError::UnexpectedQuestionMarkInsteadOfTagName, loc);
                            self.stream_prev();
                            self.state = State::BogusComment;
                        }
                        StreamEnd => {
                            self.parse_error(ParserError::EofBeforeTagName, loc);
                            self.consume('<');
                            self.state = State::Data;
                        }
                        _ => {
                            self.parse_error(ParserError::InvalidFirstCharacterOfTagName, loc);
                            self.consume('<');
                            self.stream_prev();
                            self.state = State::Data;
                        }
                    }
                }
                State::EndTagOpen => {
                    let loc = self.get_location();
                    let c = self.read_char();
                    match c {
                        Ch(ch) if ch.is_ascii_alphabetic() => {
                            self.current_token = Some(Token::EndTag {
                                name: String::new(),
                                is_self_closing: false,
                                location: self.last_token_location,
                            });
                            self.stream_prev();
                            self.state = State::TagName;
                        }
                        Ch('>') => {
                            self.parse_error(ParserError::MissingEndTagName, loc);
                            self.state = State::Data;
                        }
                        StreamEnd => {
                            self.parse_error(ParserError::EofBeforeTagName, loc);
                            self.consume('<');
                            self.consume('/');
                            self.state = State::Data;
                        }
                        _ => {
                            self.parse_error(ParserError::InvalidFirstCharacterOfTagName, loc);
                            self.current_token = Some(Token::Comment {
                                comment: String::new(),
                                location: self.last_token_location,
                            });
                            self.stream_prev();
                            self.state = State::BogusComment;
                        }
                    }
                }
                State::TagName => {
                    let loc = self.get_location();
                    let c = self.read_char();
                    match c {
                        Ch(CHAR_TAB | CHAR_LF | CHAR_FF | CHAR_SPACE) => {
                            self.state = State::BeforeAttributeName;
                        }
                        Ch('/') => self.state = State::SelfClosingStart,
                        Ch('>') => {
                            self.emit_current_token();
                            self.state = State::Data;
                        }
                        Ch(ch @ 'A'..='Z') => self.add_to_token_name(to_lowercase!(ch)),
                        Ch(CHAR_NUL) => {
                            self.parse_error(ParserError::UnexpectedNullCharacter, loc);
                            self.add_to_token_name(CHAR_REPLACEMENT);
                        }
                        StreamEnd => {
                            self.parse_error(ParserError::EofInTag, loc);
                            self.state = State::Data;
                        }
                        _ => self.add_to_token_name(c.into()),
                    }
                }
                _ => {

                }
            }
        }
    }

    fn add_to_token_name(&mut self, c: char) {
        match &mut self.current_token {
            Some(Token::StartTag { name, .. } | Token::EndTag { name, .. }) => {
                name.push(c);
            }
            Some(Token::DocType { name, .. }) => {
                // DOCTYPE can have an optional name
                match name {
                    Some(ref mut string) => string.push(c),
                    None => *name = Some(c.to_string()),
                }
            }
            _ => {}
        }
    }

    fn emit_current_token(&mut self) {
        if let Some(t) = self.current_token.take() {
            self.emit_token(t);
        }
    }

    fn emit_token(&mut self, token: Token) {
        // Save the start token name if we are pushing it. This helps us in detecting matching tags.
        if let Token::StartTag { name, .. } = &token {
            self.last_start_token = String::from(name);
        }

        // If there is any consumed data, emit this first as a text token
        if self.has_consumed_data() {
            let value = self.get_consumed_str().to_string();

            self.token_queue.push(Token::Text {
                text: value.to_string(),
                location: self.last_token_location,
            });

            self.clear_consume_buffer();
        }

        self.token_queue.push(token);
    }

    pub(crate) fn consume(&mut self, c: char) {
        // Add c to the current token data
        self.consumed.push(c);
    }

    pub fn get_consumed_str(&self) -> &str {
        &self.consumed
    }

    pub fn has_consumed_data(&self) -> bool {
        !self.consumed.is_empty()
    }

    pub(crate) fn clear_consume_buffer(&mut self) {
        self.consumed.clear();
    }

    fn read_char(&mut self) -> Character {
        let loc = self.get_location();
        let mut c = self.stream_read_and_next();

        match c {
            Character::Surrogate(..) => {
                self.parse_error(ParserError::SurrogateInInputStream, loc);
                c = Ch(CHAR_REPLACEMENT);
            }
            Ch(c) if self.is_control_char(c as u32) => {
                self.parse_error(ParserError::ControlCharacterInInputStream, loc);
            }
            Ch(c) if self.is_noncharacter(c as u32) => {
                self.parse_error(ParserError::NoncharacterInInputStream, loc);
            }
            _ => {}
        }

        tracing::debug!("stream_read(): {:?}", c);

        c
    }

    pub(crate) fn parse_error(&mut self, message: ParserError, location: Location) {
        self.error_logger.borrow_mut().add_error(location, message.as_str());
    }

    fn stream_read_and_next(&mut self) -> Character {
        let c = self.stream.read_and_next();
        self.last_char = c;
        self.location_handler.inc(c);
        c
    }

    fn stream_prev(&mut self) {
        if self.last_char == StreamEnd {
            return;
        }

        self.location_handler.dec();
        self.stream.prev();
    }

    pub(crate) fn is_noncharacter(&self, num: u32) -> bool {
        (0xFDD0..=0xFDEF).contains(&num)
            || [
                0xFFFE, 0xFFFF, 0x1FFFE, 0x1FFFF, 0x2FFFE, 0x2FFFF, 0x3FFFE, 0x3FFFF, 0x4FFFE, 0x4FFFF, 0x5FFFE,
                0x5FFFF, 0x6FFFE, 0x6FFFF, 0x7FFFE, 0x7FFFF, 0x8FFFE, 0x8FFFF, 0x9FFFE, 0x9FFFF, 0xAFFFE, 0xAFFFF,
                0xBFFFE, 0xBFFFF, 0xCFFFE, 0xCFFFF, 0xDFFFE, 0xDFFFF, 0xEFFFE, 0xEFFFF, 0xFFFFE, 0xFFFFF, 0x10FFFE,
                0x10FFFF,
            ]
            .contains(&num)
    }

    pub(crate) fn is_control_char(&self, num: u32) -> bool {
        // White spaces are ok
        if [0x0009, 0x000A, 0x000C, 0x000D, 0x0020].contains(&num) {
            return false;
        }

        (0x0001..=0x001F).contains(&num) || (0x007F..=0x009F).contains(&num)
    }
}
