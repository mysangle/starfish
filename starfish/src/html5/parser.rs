use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    html5::{
        node::HTML_NAMESPACE,
        parser::errors::{ErrorLogger, ParserError},
        tokenizer::{token::Token, ParserData, Tokenizer},
    },
    interface::{
        config::HasDocument,
        document::{Document, DocumentType},
        node::{ElementDataType, Node},
        html5::ParserOptions,
    },
    shared::{
        byte_stream::{ByteStream, Location},
        document::DocumentHandle,
        node::NodeId,
        types::{ParseError, Result},
    },
};

pub mod errors;
#[macro_use]
mod helper;

#[derive(Debug, Copy, Clone, PartialEq)]
enum InsertionMode {
    Initial,
    BeforeHtml,
}

macro_rules! get_node_by_id {
    ($doc_handle:expr, $id:expr) => {
        $doc_handle
            .get()
            .node_by_id($id)
            .expect("Node not found")
            // @todo: clone or not?
            .clone()
    };
}

macro_rules! get_element_data {
    ($node:expr) => {
        $node.get_element_data().expect("Node is not an element node")
    };
}

macro_rules! current_node {
    ($self:expr) => {{
        let current_node_idx = $self.open_elements.last().unwrap_or_default();
        $self
            .document
            .get()
            .node_by_id(*current_node_idx)
            .expect("Current node not found")
            // @todo: clone or not?
            .clone()
    }};
}

pub struct Html5ParserOptions {
    pub scripting_enabled: bool,
}

impl ParserOptions for Html5ParserOptions {
    fn new(scripting: bool) -> Self {
        Self {
            scripting_enabled: scripting,
        }
    }
}

impl Default for Html5ParserOptions {
    fn default() -> Self {
        Self {
            scripting_enabled: true,
        }
    }
}

enum DispatcherMode {
    Foreign,
    Html,
}

pub struct Html5Parser<'tokens, C: HasDocument> {
    tokenizer: Tokenizer<'tokens>,
    insertion_mode: InsertionMode,
    current_token: Token,
    scripting_enabled: bool,
    reprocess_token: bool,
    open_elements: Vec<NodeId>,
    document: DocumentHandle<C>,
    is_fragment_case: bool,
    error_logger: Rc<RefCell<ErrorLogger>>,
    ignore_lf: bool,
    token_queue: Vec<Token>,
    parser_finished: bool,
    context_node_id: Option<NodeId>,
    context_doc: Option<DocumentHandle<C>>,
}

impl<C: HasDocument> crate::interface::html5::Html5Parser<C> for Html5Parser<'_, C> {
    type Options = Html5ParserOptions;

    fn parse(stream: &mut ByteStream, doc: DocumentHandle<C>, opts: Option<Self::Options>) -> Result<Vec<ParseError>> {
        Self::parse_document(stream, doc, opts)
    }
}

impl<'a, C: HasDocument> Html5Parser<'a, C> {
    fn init(
        tokenizer: Tokenizer<'a>,
        document: DocumentHandle<C>,
        error_logger: Rc<RefCell<ErrorLogger>>,
        options: Option<Html5ParserOptions>,
    ) -> Self {
        Self {
            tokenizer,
            insertion_mode: InsertionMode::Initial,
            current_token: Token::Eof {
                location: Location::default(),
            },
            scripting_enabled: options.unwrap_or_default().scripting_enabled,
            reprocess_token: false,
            open_elements: Vec::new(),
            document,
            is_fragment_case: false,
            error_logger,
            ignore_lf: false,
            token_queue: vec![],
            parser_finished: false,
            context_node_id: None,
            context_doc: None,
        }
    }

    pub fn parse_document(
        stream: &mut ByteStream,
        document: DocumentHandle<C>,
        options: Option<Html5ParserOptions>,
    ) -> Result<Vec<ParseError>> {
        let error_logger = Rc::new(RefCell::new(ErrorLogger::new()));

        // let t_id = match &document.get().url() {
        //     Some(url) => timing_start!("html5.parse", url.as_str()),
        //     None => timing_start!("html5.parse", "unknown"),
        // };
        let tokenizer = Tokenizer::new(stream, None, error_logger.clone(), Location::default());
        let mut parser = Html5Parser::init(tokenizer, document, error_logger, options);

        let ret = parser.do_parse();
        //timing_stop!(t_id);

        ret
    }

    fn do_parse(&mut self) -> Result<Vec<ParseError>> {
        let mut dispatcher_mode = DispatcherMode::Html;

        loop {
            // When the parser is signalled to finish, we break our main parser loop
            if self.parser_finished {
                break;
            }

            // If reprocess_token is true, we should process the same token again
            if !self.reprocess_token {
                self.current_token = self.fetch_next_token();
                tracing::info!("{}", self.current_token);

                // If we reprocess a given token, the dispatcher mode should stay the same and
                // should not be re-evaluated
                dispatcher_mode = self.select_dispatch_mode();
            }

            self.reprocess_token = false;

            // Check how we should dispatch the token, and dispatch to the correct function
            match dispatcher_mode {
                DispatcherMode::Foreign => {
                    self.process_foreign_content();
                }
                DispatcherMode::Html => {
                    self.process_html_content();
                }
            }
        }

        let result = Ok(self.error_logger.borrow().get_errors().clone());
        result
    }

    fn select_dispatch_mode(&self) -> DispatcherMode {
        if self.open_elements.is_empty() {
            return DispatcherMode::Html;
        }
        return DispatcherMode::Html;
    }

    fn process_foreign_content(&mut self) {

    }

    fn process_html_content(&mut self) {
        if self.ignore_lf {
            if let Token::Text { text: value, location } = &self.current_token {
                if value.starts_with('\n') {
                    // We don't need to skip 1 char, but we can skip 1 byte, as we just checked for \n
                    self.current_token = Token::Text {
                        text: value.chars().skip(1).collect::<String>(),
                        location: *location,
                    };
                }
            }
            self.ignore_lf = false;
        }

        // match self.insertion_mode {
        //     InsertionMode::Initial => {
        //         let mut anything_else = false;

        //         match &self.current_token.clone() {
        //             Token::Text { text: value, .. } if self.current_token.is_mixed() => {
        //                 let tokens = self.split_mixed_token(value);
        //                 self.tokenizer.insert_tokens_at_queue_start(&tokens);
        //                 return;
        //             }
        //             Token::Text { .. } if self.current_token.is_empty_or_white() => {
        //                 // ignore token
        //             }
        //             Token::Comment { .. } => {
        //                 self.insert_comment_element(&self.current_token.clone(), Some(NodeId::root()));
        //             }
        //             Token::DocType {
        //                 name,
        //                 pub_identifier,
        //                 sys_identifier,
        //                 force_quirks,
        //                 ..
        //             } => {
        //                 if name.is_some() && name.as_ref().unwrap() != "html"
        //                     || pub_identifier.is_some()
        //                     || (sys_identifier.is_some() && sys_identifier.as_ref().unwrap() != "about:legacy-compat")
        //                 {
        //                     self.parse_error("doctype not allowed in initial insertion mode");
        //                 }

        //                 self.insert_doctype_element(&self.current_token.clone());

        //                 if !self.is_iframesrcdoc() && !self.parser_cannot_change_mode {
        //                     self.set_quirks_mode(self.identify_quirks_mode(
        //                         name,
        //                         pub_identifier.clone(),
        //                         sys_identifier.clone(),
        //                         *force_quirks,
        //                     ));
        //                 }

        //                 self.insertion_mode = InsertionMode::BeforeHtml;
        //             }
        //             Token::StartTag { .. } => {
        //                 if !self.is_iframesrcdoc() {
        //                     self.parse_error(ParserError::ExpectedDocTypeButGotStartTag.as_str());
        //                 }
        //                 anything_else = true;
        //             }
        //             Token::EndTag { .. } => {
        //                 if !self.is_iframesrcdoc() {
        //                     self.parse_error(ParserError::ExpectedDocTypeButGotEndTag.as_str());
        //                 }
        //                 anything_else = true;
        //             }
        //             Token::Text { .. } => {
        //                 if !self.is_iframesrcdoc() {
        //                     self.parse_error(ParserError::ExpectedDocTypeButGotChars.as_str());
        //                 }
        //                 anything_else = true;
        //             }
        //             Token::Eof { .. } => anything_else = true,
        //         }

        //         if anything_else {
        //             if !self.parser_cannot_change_mode {
        //                 self.set_quirks_mode(QuirksMode::Quirks);
        //             }

        //             self.insertion_mode = InsertionMode::BeforeHtml;
        //             self.reprocess_token = true;
        //         }
        //     }
        //     InsertionMode::BeforeHtml => {

        //     }
        // }

        if self.current_token.is_eof() {
            self.stop_parsing();
        }
    }

    fn is_iframesrcdoc(&self) -> bool {
        self.document.get().doctype() == DocumentType::IframeSrcDoc
    }

    /// Fetches the next token from the tokenizer. However, if the token is a text token AND
    /// it starts with one or more whitespaces, the token is split into 2 tokens: the whitespace part
    /// and the remainder.
    fn fetch_next_token(&mut self) -> Token {
        if self.token_queue.is_empty() {
            let token = self.tokenizer.next_token(self.parser_data()).expect("tokenizer error");

            if let Token::Text { text: value, location } = token {
                self.token_queue.push(Token::Text { text: value, location });
            } else {
                // Simply return the token
                return token;
            }
        }

        let token = self.token_queue.first().cloned();
        self.token_queue.remove(0);

        token.expect("no token found")
    }

    fn parser_data(&self) -> ParserData {
        if self.open_elements.is_empty() {
            return ParserData {
                adjusted_node_namespace: HTML_NAMESPACE.to_string(),
            };
        }

        let node = self.get_adjusted_current_node();
        let data = get_element_data!(node);
        ParserData {
            adjusted_node_namespace: data.namespace().to_string(),
        }
    }

    fn get_adjusted_current_node(&self) -> <C::Document as Document<C>>::Node {
        if self.is_fragment_case && self.open_elements.len() == 1 {
            // fragment case
            return get_node_by_id!(
                self.context_doc.clone().expect("context doc not found"),
                self.context_node_id.expect("context node not found")
            )
            .clone();
        }

        current_node!(self)
    }

    fn create_node(&self, token: &Token, namespace: &str) -> C::Node {
        match token {
            Token::DocType {
                name,
                force_quirks: _,
                pub_identifier,
                sys_identifier,
                location,
            } => C::Document::new_doctype_node(
                self.document.clone(),
                &name.clone().unwrap_or_default(),
                match pub_identifier {
                    Some(value) => Some(value.as_str()),
                    None => None,
                },
                match sys_identifier {
                    Some(value) => Some(value.as_str()),
                    None => None,
                },
                *location,
            ),
            Token::StartTag {
                name,
                attributes,
                location,
                ..
            } => C::Document::new_element_node(
                self.document.clone(),
                name,
                namespace.into(),
                attributes.clone(),
                *location,
            ),
            Token::EndTag { name, location, .. } => {
                C::Document::new_element_node(self.document.clone(), name, namespace.into(), HashMap::new(), *location)
            }
            Token::Comment {
                comment: value,
                location,
                ..
            } => C::Document::new_comment_node(self.document.clone(), value, *location),
            Token::Text {
                text: value, location, ..
            } => C::Document::new_text_node(self.document.clone(), value.as_str(), *location),
            Token::Eof { .. } => {
                panic!("EOF token not allowed");
            }
        }
    }

    fn stop_parsing(&mut self) {
        self.parser_finished = true;
    }

    pub fn get_parse_errors(&self) -> Vec<ParseError> {
        self.error_logger.borrow().get_errors().clone()
    }

    fn parse_error(&self, message: &str) {
        self.error_logger
            .borrow_mut()
            .add_error(self.current_token.get_location(), message);
    }
}
