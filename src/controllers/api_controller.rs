use std::collections::HashMap;
use crate::controllers::Controller;
use crate::http_server::request::Request;
use crate::http_server::response::data_response::DataResponse;
use crate::http_server::response::Response;
use crate::http_server::status::Status;

pub struct ApiController {

}

impl ApiController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for ApiController {
    fn handle(&self, request: &Request) -> Box<dyn Response> {
        Box::new(DataResponse::new(
            Status::Ok200,
            HashMap::from([("Content-Type".to_string(), "application/json".to_string())]),
            Some("{\"foo\":\"bar\"}".to_string())
        ))
    }

    fn path(&self) -> String {
        "/api/test".to_string()
    }
}
