use std::collections::HashMap;
use std::fs;
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use crate::http::parse::parse_http_stream;
use crate::http::request::Request;
use crate::http::response::Response;

const ADDRESS: &str = "127.0.0.1:7878";

pub fn start_http_server() {
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind http server");
    println!("Server started!");
    println!("Listening on: http://{}", ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }

}
fn handle_connection(mut stream: TcpStream) {
    println!("========[ Incoming connection! ]========");

    let request = parse_http_stream(&stream);

    println!("Request: {:#?}", request);

    let response = match_request(&request);
    stream.write_all(response.to_response_data_string().as_bytes()).expect("Failed to send response!");
    stream.flush().expect("Failed to flush stream!");
}

fn match_request(request: &Request) -> Response {
    match request.path.as_str() {
        "/" => Response::ok(HashMap::new(), Some(fs::read_to_string("public/index.html").expect("Failed to read index.html"))),
        _ => Response::notFound(HashMap::new(), Some(fs::read_to_string("public/404.html").expect("Failed to read index.html")))
    }
}
