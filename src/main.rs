use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::str::CharIndices;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::http_server::request::Request;
use crate::http_server::response::Response;
use crate::http_server::server::start_http_server;
use crate::controllers::api_controller::ApiController;
use crate::controllers::character_index_controller::CharacterIndexController;
use crate::controllers::Controller;
use crate::controllers::controller_list::ControllerList;
use crate::controllers::file_controller::FileController;
use crate::controllers::get_character_controller::GetCharacterController;
use crate::controllers::param_controller::ParamController;
use crate::http_server::method::Method;
use crate::http_server::response::file_response::FileResponse;

mod http_server;
mod thread_pool;
mod controllers;

struct ControllerRoute<F>
    where
    F: FnOnce(&Request) -> dyn Response
{
    route: String,
    handle: HashMap<String, Box<F>>,
}


fn main() {
    let controller_array: Vec<(Method, Box<dyn Controller + Sync + Send>)> = vec![
        (Method::GET, Box::new(FileController::new())),
        (Method::GET, Box::new(ApiController::new())),

        (Method::GET, Box::new(CharacterIndexController::new())),
        (Method::GET, Box::new(GetCharacterController::new())),

        (Method::GET, Box::new(ParamController::new())),
    ];
    let controller_list = ControllerList::from(controller_array);

    start_http_server(Arc::new(controller_list));

    println!("Shutting down...")
}

pub fn match_request(request: &mut Request) -> Vec<u8>
{
    println!("Request: {:#?}", request);

    match request.path.as_str() {
        "/" => FileResponse::ok("public/index.html".to_string()).to_response_data_bytes(),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            FileResponse::ok("public/index.html".to_string()).to_response_data_bytes()
        },
        _ => FileResponse::ok("public/404.html".to_string()).to_response_data_bytes(),
    }
}

