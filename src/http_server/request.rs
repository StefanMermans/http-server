use std::collections::HashMap;
use crate::http_server::method::Method;

#[derive(Debug)]
pub struct Request {
    pub protocol: String,
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub content_length: usize,
    pub parameters: HashMap<String, String>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: Method::GET,
            path: "".to_string(),
            protocol: "".to_string(),
            headers: HashMap::new(),
            content: None,
            content_length: 0,
            parameters: HashMap::new(),
        }
    }
}