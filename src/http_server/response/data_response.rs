use std::collections::HashMap;
use crate::http_server::response::Response;
use crate::http_server::status::Status;

pub struct DataResponse {
    status: Status,
    headers: HashMap<String, String>,
    content: Option<String>,
}

impl DataResponse {
    pub fn new(status: Status, headers: HashMap<String, String>, content: Option<String>) -> Self {
        let mut response = DataResponse {
            status,
            headers,
            content: Some(String::new()),
        };
        response.set_content(content);

        response
    }

    pub fn json(content: String) -> Self {
        DataResponse::new(
            Status::Ok200,
            HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            Some(content),
        )
    }

    pub fn default() -> Self {
        DataResponse::new(Status::Ok200, HashMap::new(), None)
    }

    pub fn ok(headers: HashMap<String, String>, content: Option<String>) -> Self {
        DataResponse::new(Status::Ok200, headers, content)
    }

    pub fn not_found(headers: HashMap<String, String>, content: Option<String>) -> Self {
        DataResponse::new(Status::NotFound404, headers, content)
    }

    pub fn set_content(&mut self, content: Option<String>) {
        if let Some(data) = content {
            self.headers.insert("Content-Length".to_string(), data.len().to_string());
            self.content = Some(data);

            return;
        }

        self.headers.remove("Content-Length");
    }
}

impl Response for DataResponse {
    fn to_response_data_bytes(&self) -> Vec<u8> {
        let mut content: Vec<u8> = match &self.content {
            None => vec![],
            Some(content) => Vec::from(content.as_bytes()),
        };

        self.http_bytes(
            &self.headers_string(&self.status, &self.headers),
            &mut content,
        )
    }
}
