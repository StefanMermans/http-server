use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::http_server::request::Request;
use crate::http_server::response::Response;
use crate::http_server::server::start_http_server;
use crate::controllers::{Controller, file_controller};
use crate::controllers::api_controller::ApiController;
use crate::controllers::file_controller::FileController;
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
    let routes_array: [(String, Box<dyn Controller + Sync + Send>); 2] = [
        ("".to_string(), Box::new(FileController::new())),
        ("".to_string(), Box::new(ApiController::new()))
    ];
    let routes: HashMap<String, Box<dyn Controller + Sync + Send>> = HashMap::from(routes_array);

    start_http_server(Arc::new(routes));

    println!("Shutting down...")
}

pub fn match_request(request: &mut Request) -> Vec<u8>
{
    println!("Request: {:#?}", request);

    // let controller = FileController {};
    // controller.show(request).to_response_data_bytes()

    match request.path.as_str() {
        "/" => FileResponse::ok("public/index.html".to_string()).to_response_data_bytes(),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            FileResponse::ok("public/index.html".to_string()).to_response_data_bytes()
        },
        _ => FileResponse::ok("public/404.html".to_string()).to_response_data_bytes(),
    }
}

