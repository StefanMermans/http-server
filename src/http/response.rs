use std::collections::HashMap;
use crate::http::status::Status;

pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    content: Option<String>,
}

impl Response {
    pub fn new(status: Status, headers: HashMap<String, String>, content: Option<String>) -> Self {
        let mut response = Response {
            status,
            headers,
            content: Some(String::new()),
        };
        response.set_content(content);

        response
    }

    pub fn ok(headers: HashMap<String, String>, content: Option<String>) -> Self {
        Response::new(Status::Ok200, headers, content)
    }

    pub fn notFound(headers: HashMap<String, String>, content: Option<String>) -> Self {
        Response::new(Status::NotFound404, headers, content)
    }

    pub fn set_content(&mut self, content: Option<String>)  {
        if let Some(data) = content {
            self.headers.insert("Content-Length".to_string(), data.len().to_string());
            self.content = Some(data);

            return;
        }

        self.headers.remove("Content-Length");
    }

    pub fn to_response_data_string(&self) -> String {
        let mut data = "".to_string();

        data += format!("HTTP/1.1 {}\r\n", &self.status.as_str()).as_str();

        for (key, value) in self.headers.iter() {
            data += format!("{}: {}\r\n", key, value).as_str();
        }

        data += "\r\n";

        if let Some(content) = &self.content {
            data += content
        }

        data
    }
}
