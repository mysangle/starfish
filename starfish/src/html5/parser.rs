use std::{cell::RefCell, rc::Rc};

use crate::{
    html5::{
        node::HTML_NAMESPACE,
        parser::errors::ErrorLogger,
        tokenizer::{token::Token, ParserData, Tokenizer},
    },
    interface::{
        config::HasDocument,
        document::Document,
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
    current_token: Token,
    scripting_enabled: bool,
    reprocess_token: bool,
    open_elements: Vec<NodeId>,
    document: DocumentHandle<C>,
    is_fragment_case: bool,
    error_logger: Rc<RefCell<ErrorLogger>>,
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
            current_token: Token::Eof {
                location: Location::default(),
            },
            scripting_enabled: options.unwrap_or_default().scripting_enabled,
            reprocess_token: false,
            open_elements: Vec::new(),
            document,
            is_fragment_case: false,
            error_logger,
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
        return DispatcherMode::Html;
    }

    fn process_foreign_content(&mut self) {

    }

    fn process_html_content(&mut self) {
        if self.current_token.is_eof() {
            self.stop_parsing();
        }
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

    fn stop_parsing(&mut self) {
        self.parser_finished = true;
    }
}
