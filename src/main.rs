use crate::http::request::Request;
use crate::http::response::file_response::FileResponse;
use crate::http::response::Response;
use crate::http::server::start_http_server;

mod http;

fn main() {
    start_http_server(&match_request);
}

fn match_request(request: &mut Request) -> Vec<u8>
{
    println!("Request: {:#?}", request);

    match request.path.as_str() {
        "/" => FileResponse::ok("public/index.html".to_string()).to_response_data_bytes(),
        "/sleep" => {

            FileResponse::ok("public/index.html".to_string()).to_response_data_bytes()
        },
        _ => FileResponse::ok("public/404.html".to_string()).to_response_data_bytes(),

    }
}

