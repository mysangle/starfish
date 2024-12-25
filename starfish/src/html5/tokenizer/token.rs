use std::collections::HashMap;

use crate::shared::byte_stream::Location;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    DocType {
        name: Option<String>,
        force_quirks: bool,
        pub_identifier: Option<String>,
        sys_identifier: Option<String>,
        location: Location,
    },
    StartTag {
        name: String,
        is_self_closing: bool,
        attributes: HashMap<String, String>,
        location: Location,
    },
    EndTag {
        name: String,
        is_self_closing: bool,
        location: Location,
    },
    Comment {
        comment: String,
        location: Location,
    },
    Text {
        text: String,
        location: Location,
    },
    Eof {
        location: Location,
    },
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Token::DocType {
                name,
                pub_identifier,
                sys_identifier,
                ..
            } => {
                let mut result = format!("<!DOCTYPE {}", name.clone().unwrap_or_default());
                if let Some(pub_id) = pub_identifier {
                    result.push_str(&format!(r#" PUBLIC "{pub_id}""#));
                }
                if let Some(sys_id) = sys_identifier {
                    result.push_str(&format!(r#" SYSTEM "{sys_id}""#));
                }
                result.push_str(" />");
                write!(f, "{result}")
            }
            Token::Comment { comment: value, .. } => write!(f, "<!-- {value} -->"),
            Token::Text { text: value, .. } => write!(f, "{value}"),
            Token::StartTag {
                name,
                is_self_closing,
                attributes,
                ..
            } => {
                let mut result = format!("<{name}");
                for (key, value) in attributes {
                    result.push_str(&format!(r#" {key}="{value}""#));
                }
                if *is_self_closing {
                    result.push_str(" /");
                }
                result.push('>');
                write!(f, "{result}")
            }
            Token::EndTag {
                name, is_self_closing, ..
            } => write!(f, "</{}{}>", name, if *is_self_closing { "/" } else { "" }),
            Token::Eof { .. } => write!(f, "EOF"),
        }
    }
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Token::Eof { .. })
    }
}
