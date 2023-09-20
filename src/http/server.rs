use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use crate::http::parse::parse_http_stream;
use crate::http::request::Request;

const ADDRESS: &str = "127.0.0.1:7878";

pub fn start_http_server<F>(on_request: &F)
    where
    F: Fn(&mut Request) -> Vec<u8>
{
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind http server");
    println!("Server started!");
    println!("Listening on: http://{}", ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, on_request);
    }
}

fn handle_connection<F>(mut stream: TcpStream, on_request: &F)
    where
    F: Fn(&mut Request) -> Vec<u8>
{
    let mut request = parse_http_stream(&stream);
    let response = on_request(&mut request);

    stream.write_all(response.as_slice()).expect("Failed to send response!");
}
