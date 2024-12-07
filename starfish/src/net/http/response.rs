use std::collections::HashMap;

use crate::net::http::headers::Headers;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub status_text: String,
    pub version: String,
    pub headers: Headers,
    pub cookies: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new() -> Response {
        Self {
            status: 0,
            status_text: "".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: Default::default(),
            cookies: Default::default(),
            body: vec![],
        }
    }
}

impl From<Vec<u8>> for Response {
    fn from(body: Vec<u8>) -> Self {
        Self {
            status: 200,
            status_text: "OK".to_string(),
            version: "HTTP/1.1".to_string(),
            headers: Default::default(),
            cookies: Default::default(),
            body,
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}
