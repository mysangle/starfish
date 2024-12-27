use std::{
    cell::RefCell,
    char::REPLACEMENT_CHARACTER,
    fmt::{self, Debug, Display, Formatter}
};

pub const CHAR_LF: char = '\u{000A}';
pub const CHAR_CR: char = '\u{000D}';

/// Encoding defines the way the buffer stream is read, as what defines a "character".
#[derive(PartialEq)]
pub enum Encoding {
    /// Unknown encoding. Won't read anything from the stream until the encoding is set
    UNKNOWN,
    /// Stream is of single byte ASCII chars (0-255)
    ASCII,
    /// Stream is of UTF8 characters
    UTF8,
    // Stream consists of 16-bit UTF characters (Little Endian)
    UTF16LE,
    // Stream consists of 16-bit UTF characters (Big Endian)
    UTF16BE,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Character {
    /// Standard UTF character
    Ch(char),
    /// Surrogate character (since they cannot be stored in char)
    Surrogate(u16),
    /// Stream buffer empty and closed
    StreamEnd,
    /// Stream buffer empty (but not closed)
    StreamEmpty,
}

use Character::*;

impl From<&Character> for char {
    fn from(c: &Character) -> Self {
        match c {
            Ch(c) => *c,
            Surrogate(..) => 0x0000 as char,
            StreamEmpty | StreamEnd => 0x0000 as char,
        }
    }
}

impl From<Character> for char {
    fn from(c: Character) -> Self {
        match c {
            Ch(c) => c,
            Surrogate(..) => 0x0000 as char,
            StreamEmpty | StreamEnd => 0x0000 as char,
        }
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ch(ch) => write!(f, "{ch}"),
            Surrogate(surrogate) => write!(f, "U+{surrogate:04X}"),
            StreamEnd => write!(f, "StreamEnd"),
            StreamEmpty => write!(f, "StreamEmpty"),
        }
    }
}

impl Character {
    pub fn slice_to_string(v: Vec<Character>) -> String {
        v.iter().map(char::from).collect()
    }
}

/// Configuration structure for a bytestream.
pub struct Config {
    /// Treat any CRLF pairs as a single LF
    pub cr_lf_as_one: bool,
    /// Replace any CR (without a pairing LF) with LF
    pub replace_cr_as_lf: bool,
    /// Are high ascii characters read as-is or converted to a replacement character
    pub replace_high_ascii: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cr_lf_as_one: true,
            replace_cr_as_lf: false,
            replace_high_ascii: false,
        }
    }
}

pub trait Stream {
    fn read(&self) -> Character;
    fn read_and_next(&self) -> Character;
    fn look_ahead(&self, offset: usize) -> Character;
    fn next(&self);
    fn next_n(&self, offset: usize);
    fn prev(&self);
    fn prev_n(&self, n: usize);
    fn seek_bytes(&self, offset: usize);
    fn tell_bytes(&self) -> usize;
    fn get_slice(&self, len: usize) -> Vec<Character>;
    fn close(&mut self);
    fn closed(&self) -> bool;
    fn exhausted(&self) -> bool;
    fn eof(&self) -> bool;
}

pub struct ByteStream {
    /// Actual buffer stream in u8 bytes
    buffer: Vec<u8>,
    /// Current position in the stream
    buffer_pos: RefCell<usize>,
    /// True when the buffer is empty and not yet have a closed stream
    closed: bool,
    /// Current encoding
    encoding: Encoding,
    // Configuration for the stream
    config: Config,
}

impl ByteStream {
    #[must_use]
    pub fn new(encoding: Encoding, config: Option<Config>) -> Self {
        Self {
            buffer: Vec::new(),
            buffer_pos: RefCell::new(0),
            closed: false,
            encoding,
            config: config.unwrap_or_default(),
        }
    }

    pub fn read_from_str(&mut self, s: &str, _encoding: Option<Encoding>) {
        self.buffer = Vec::from(s.as_bytes());
        self.reset_stream();
    }

    pub fn close(&mut self) {
        self.closed = true;
    }

    fn reset_stream(&self) {
        let mut pos = self.buffer_pos.borrow_mut();
        *pos = 0;
    }

    fn read_with_length(&self) -> (Character, usize) {
        if self.eof() || self.buffer.is_empty() || *self.buffer_pos.borrow() >= self.buffer.len() {
            if self.closed {
                return (StreamEnd, 0);
            }
            return (StreamEmpty, 0);
        }

        let buf_pos = self.buffer_pos.borrow();
        match self.encoding {
            Encoding::UNKNOWN => {
                todo!("Unknown encoding. Please detect encoding first");
            }
            Encoding::ASCII => {
                if *buf_pos >= self.buffer.len() {
                    if self.closed {
                        return (StreamEnd, 0);
                    }
                    return (StreamEmpty, 0);
                }

                if self.config.replace_high_ascii && self.buffer[*buf_pos] > 127 {
                    (Ch('?'), 1)
                } else {
                    (Ch(self.buffer[*buf_pos] as char), 1)
                }
            }
            Encoding::UTF8 => {
                let first_byte = self.buffer[*buf_pos];
                let width = utf8_char_width(first_byte);

                if *buf_pos + width > self.buffer.len() {
                    return (StreamEmpty, self.buffer.len() - *buf_pos);
                }

                let ch = match width {
                    1 => first_byte as u32,
                    2 => ((first_byte as u32 & 0x1F) << 6) | (self.buffer[*buf_pos + 1] as u32 & 0x3F),
                    3 => {
                        ((first_byte as u32 & 0x0F) << 12)
                            | ((self.buffer[*buf_pos + 1] as u32 & 0x3F) << 6)
                            | (self.buffer[*buf_pos + 2] as u32 & 0x3F)
                    }
                    4 => {
                        ((first_byte as u32 & 0x07) << 18)
                            | ((self.buffer[*buf_pos + 1] as u32 & 0x3F) << 12)
                            | ((self.buffer[*buf_pos + 2] as u32 & 0x3F) << 6)
                            | (self.buffer[*buf_pos + 3] as u32 & 0x3F)
                    }
                    _ => 0xFFFD, // Invalid UTF-8 byte sequence
                };

                if ch > 0x10FFFF || (ch > 0xD800 && ch <= 0xDFFF) {
                    (Surrogate(ch as u16), width)
                } else {
                    (char::from_u32(ch).map_or(Ch(REPLACEMENT_CHARACTER), Ch), width)
                }
            }
            Encoding::UTF16LE => {
                if *buf_pos + 1 < self.buffer.len() {
                    let code_unit = u16::from_le_bytes([self.buffer[*buf_pos], self.buffer[*buf_pos + 1]]);
                    (
                        char::from_u32(u32::from(code_unit)).map_or(Ch(REPLACEMENT_CHARACTER), Ch),
                        2,
                    )
                } else {
                    (StreamEmpty, 1)
                }
            }
            Encoding::UTF16BE => {
                if *buf_pos + 1 < self.buffer.len() {
                    let code_unit = u16::from_be_bytes([self.buffer[*buf_pos], self.buffer[*buf_pos + 1]]);
                    (
                        char::from_u32(u32::from(code_unit)).map_or(Ch(REPLACEMENT_CHARACTER), Ch),
                        2,
                    )
                } else {
                    (StreamEmpty, 1)
                }
            }
        }
    }

    fn move_back(&self, n: usize) {
        let mut pos = self.buffer_pos.borrow_mut();

        match self.encoding {
            Encoding::ASCII => {
                if *pos > n {
                    *pos -= n;
                } else {
                    *pos = 0;
                }
            }
            Encoding::UTF8 => {
                let mut n = n;
                while n > 0 && *pos > 0 {
                    *pos -= 1;

                    if self.buffer[*pos] & 0b1100_0000 != 0b1000_0000 {
                        n -= 1;
                    }
                }
            }
            Encoding::UTF16LE => {
                if *pos > n * 2 {
                    *pos -= n * 2;
                } else {
                    *pos = 0;
                }
            }
            Encoding::UTF16BE => {
                if *pos > n * 2 {
                    *pos -= n * 2;
                } else {
                    *pos = 0;
                }
            }
            _ => {}
        }
    }
}

impl Stream for ByteStream {
    fn read(&self) -> Character {
        let (ch, _) = self.read_with_length();
        ch
    }

    fn read_and_next(&self) -> Character {
        let (ch, len) = self.read_with_length();

        {
            let mut pos = self.buffer_pos.borrow_mut();
            *pos += len;
        }

        // Make sure we skip the CR if it is followed by a LF
        if self.config.cr_lf_as_one && ch == Ch(CHAR_CR) && self.read() == Ch(CHAR_LF) {
            self.next();
            return Ch(CHAR_LF);
        }

        // Replace CR with LF if it is not followed by a LF
        if self.config.replace_cr_as_lf && ch == Ch(CHAR_CR) && self.read() != Ch(CHAR_LF) {
            return Ch(CHAR_LF);
        }

        ch
    }

    fn look_ahead(&self, offset: usize) -> Character {
        if self.buffer.is_empty() {
            return StreamEnd;
        }

        let original_pos = *self.buffer_pos.borrow();

        self.next_n(offset);
        let ch = self.read();

        let mut pos = self.buffer_pos.borrow_mut();
        *pos = original_pos;

        ch
    }

    fn next(&self) {
        self.next_n(1);
    }

    /// Returns the n'th character in the stream
    fn next_n(&self, offset: usize) {
        for _ in 0..offset {
            let (_, len) = self.read_with_length();
            if len == 0 {
                return;
            }

            let mut pos = self.buffer_pos.borrow_mut();
            *pos += len;
        }
    }

    fn prev(&self) {
        self.prev_n(1);
    }

    fn prev_n(&self, n: usize) {
        // No need for extra checks, so we can just move back n characters
        if !self.config.cr_lf_as_one {
            self.move_back(n);
            return;
        }

        // We need to loop n times, as we might encounter CR/LF pairs we need to take into account
        for _ in 0..n {
            self.move_back(1);

            if self.config.cr_lf_as_one && self.read() == Ch(CHAR_CR) && self.look_ahead(1) == Ch(CHAR_LF) {
                self.move_back(1);
            }
        }
    }

    fn seek_bytes(&self, offset: usize) {
        let mut pos = self.buffer_pos.borrow_mut();
        *pos = offset;
    }

    fn tell_bytes(&self) -> usize {
        *self.buffer_pos.borrow()
    }

    fn get_slice(&self, len: usize) -> Vec<Character> {
        let current_pos = self.tell_bytes();

        let mut slice = Vec::with_capacity(len);
        for _ in 0..len {
            slice.push(self.read_and_next());
        }

        self.seek_bytes(current_pos);

        slice.clone()
    }

    fn close(&mut self) {
        self.closed = true;
    }

    fn closed(&self) -> bool {
        self.closed
    }

    fn exhausted(&self) -> bool {
        *self.buffer_pos.borrow() >= self.buffer.len()
    }

    fn eof(&self) -> bool {
        self.closed() && self.exhausted()
    }
}

/// Location holds the start position of the given element in the data source
#[derive(Clone, PartialEq, Copy)]
pub struct Location {
    /// Line number, starting with 1
    pub line: usize,
    /// Column number, starting with 1
    pub column: usize,
    /// Byte offset, starting with 0
    pub offset: usize,
}

impl Default for Location {
    /// Default to line 1, column 1
    fn default() -> Self {
        Self::new(1, 1, 0)
    }
}

impl Location {
    /// Create a new Location
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.line, self.column)
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}:{})", self.line, self.column)
    }
}

pub struct LocationHandler {
    /// The start offset of the location. Normally this is 0:0, but can be different in case of inline streams
    pub start_location: Location,
    /// The current location of the stream
    pub cur_location: Location,
    /// Stack of all column size
    column_stack: Vec<usize>,
}

impl LocationHandler {
    pub fn new(start_location: Location) -> Self {
        Self {
            start_location,
            cur_location: Location::default(),
            column_stack: Vec::new(),
        }
    }

    /// Will decrease the current location based on the current character
    pub fn dec(&mut self) {
        if self.cur_location.column > 1 {
            self.cur_location.column -= 1;
            self.cur_location.offset -= 1;
        } else if self.cur_location.line > 1 {
            self.cur_location.line -= 1;
            self.cur_location.column = self.column_stack.pop().unwrap_or(1);
            self.cur_location.offset -= 1;
        }
    }

    /// Will increase the current location based on the given character
    pub fn inc(&mut self, ch: Character) {
        match ch {
            Ch(CHAR_LF) => {
                self.column_stack.push(self.cur_location.column);
                self.cur_location.line += 1;
                self.cur_location.column = 1;
                self.cur_location.offset += 1;
            }
            Ch(_) => {
                self.cur_location.column += 1;
                self.cur_location.offset += 1;
            }
            StreamEnd | StreamEmpty => {}
            _ => {}
        }
    }
}

/// Returns the width of the given UTF8 character, which is based on the first byte
#[inline]
fn utf8_char_width(first_byte: u8) -> usize {
    if first_byte < 0x80 {
        1
    } else {
        2 + (first_byte >= 0xE0) as usize + (first_byte >= 0xF0) as usize
    }
}
