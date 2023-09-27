use crate::controllers::Controller;
use crate::http_server::request::Request;
use crate::http_server::response::data_response::DataResponse;
use crate::http_server::response::Response;

pub struct GetCharacterController {}

impl GetCharacterController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for GetCharacterController {
    fn handle(&self, request: &Request) -> Box<dyn Response> {
        Box::new(DataResponse::json("{\"id\":2,\"name\":\"Takina\"}".to_string()))
    }

    fn path(&self) -> String {
        // TODO: Path param
        "api/character/2".to_string()
    }
}