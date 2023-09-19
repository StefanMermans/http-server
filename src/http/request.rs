use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub protocol: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub content: Option<Vec<u8>>,
    pub content_length: usize,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: String::new(),
            path: "".to_string(),
            protocol: "".to_string(),
            headers: HashMap::new(),
            content: None,
            content_length: 0,
        }
    }
}