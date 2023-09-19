use std::fs;
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use crate::http::parse::parse_http;

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

    let request = parse_http(&stream);

    println!("Request: {:#?}", request);

    let status_line = "HTTP/1.1 200 OK";
    let contents =  fs::read_to_string("public/index.html").expect("Failed to read index.html");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("Failed to send response!");
}
