use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use crate::http_server::parse::parse_http_stream;
use crate::match_request;
use crate::thread_pool::ThreadPool;

const ADDRESS: &str = "127.0.0.1:7878";

pub fn start_http_server()
{
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind http_server server");
    println!("Server started!");
    println!("Listening on: http://{}", ADDRESS);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });
    }
}

fn handle_connection(mut stream: TcpStream)
{
    let mut request = parse_http_stream(&stream);
    let response = match_request(&mut request);

    stream.write_all(response.as_slice()).expect("Failed to send response!");
}
