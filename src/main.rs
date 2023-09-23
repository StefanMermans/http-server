use std::thread;
use std::time::Duration;
use crate::http_server::request::Request;
use crate::http_server::response::file_response::FileResponse;
use crate::http_server::response::Response;
use crate::http_server::server::start_http_server;

mod http_server;
mod thread_pool;

fn main() {
    start_http_server();

    println!("Shutting down...")
}

pub fn match_request(request: &mut Request) -> Vec<u8>
{
    // println!("Request: {:#?}", request);

    match request.path.as_str() {
        "/" => FileResponse::ok("public/index.html".to_string()).to_response_data_bytes(),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            FileResponse::ok("public/index.html".to_string()).to_response_data_bytes()
        },
        _ => FileResponse::ok("public/404.html".to_string()).to_response_data_bytes(),
    }
}

