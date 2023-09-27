use std::collections::HashMap;
use crate::controllers::Controller;
use crate::http_server::request::Request;
use crate::http_server::response::data_response::DataResponse;
use crate::http_server::response::Response;

pub struct NotFoundController {}

impl NotFoundController {
    pub fn new() -> Self {
        Self{}
    }
}

impl Controller for NotFoundController {
    fn handle(&self, request: &Request) -> Box<dyn Response> {
        Box::new(DataResponse::not_found(HashMap::new(), Some("Not found".to_string())))
    }

    fn path(&self) -> String {
        "".to_string()
    }
}
