use std::collections::HashMap;
use crate::http_server::status::Status;

pub mod data_response;
pub mod file_response;

pub trait Response {
    fn to_response_data_bytes(&self) -> Vec<u8>;

    fn headers_string(&self, status: &Status, headers: &HashMap<String, String>) -> String {
        let mut data = String::new();

        data += format!("HTTP/1.1 {}\r\n", &status.as_str()).as_str();

        for (key, value) in headers.iter() {
            data += format!("{}: {}\r\n", key, value).as_str();
        }

        data += "\r\n";

        data
    }

    fn http_bytes(&self, headers: &String, content: &mut Vec<u8>) -> Vec<u8> {
        let mut http_data = Vec::from(headers.as_bytes());
        http_data.append(content);

        http_data
    }
}