use crate::controllers::Controller;
use crate::http_server::request::Request;
use crate::http_server::response::data_response::DataResponse;
use crate::http_server::response::Response;

pub struct CharacterIndexController {}

impl CharacterIndexController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for CharacterIndexController {
    fn handle(&self, request: &Request) -> Box<dyn Response> {
        Box::new(DataResponse::json("[{\"id\":1,\"name\":\"Chisato\"},{\"id\":2,\"name\":\"Takina\"}]".to_string()))
    }

    fn path(&self) -> String {
        "api/character".to_string()
    }
}
