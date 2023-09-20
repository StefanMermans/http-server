use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use crate::http::parse::parse_http_stream;
use crate::match_request;

const ADDRESS: &str = "127.0.0.1:7878";

pub fn start_http_server()
{
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind http server");
    println!("Server started!");
    println!("Listening on: http://{}", ADDRESS);


    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // TODO: Create a custom threadpool. cause why not?
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut request = parse_http_stream(&stream);
    let response = match_request(&mut request);

    stream.write_all(response.as_slice()).expect("Failed to send response!");
}
