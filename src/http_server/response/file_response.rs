use std::{fs, io};
use std::collections::HashMap;
use crate::http_server::response::Response;
use crate::http_server::status::Status;

pub struct FileResponse {
    status: Status,
    filename: String,
}

impl FileResponse {
    pub fn new(status: Status, filename: String) -> Self {
        Self {
            status,
            filename,
        }
    }

    pub fn ok(filename: String) -> Self {
        Self::new(Status::Ok200, filename)
    }

    fn read_file(&self) -> io::Result<String> {
        fs::read_to_string(&self.filename)
    }
}

impl Response for FileResponse {
    fn to_response_data_bytes(&self) -> Vec<u8> {
        let mut content: Vec<u8> = Vec::from(self.read_file().expect("Failed to read file!"));

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-Length".to_string(), content.len().to_string());

        self.http_bytes(
            &self.headers_string(&self.status, &headers),
            &mut content,
        )
    }
}