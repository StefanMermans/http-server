use crate::controllers::Controller;
use crate::http_server::request::Request;
use crate::http_server::response::data_response::DataResponse;
use crate::http_server::response::Response;

pub struct ParamController {

}

impl ParamController {
    pub fn new() -> Self { Self{} }
}

impl Controller for ParamController {
    fn handle(&self, request: &Request) -> Box<dyn Response> {
        println!("params: {:#?}", &request.parameters);

        Box::new(DataResponse::json("{}".to_string()))
    }

    fn path(&self) -> String {
        "/api/param/{foo}/{bar}".to_string()
    }
}
