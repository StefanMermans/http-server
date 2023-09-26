use std::collections::HashMap;
use std::io::{Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use crate::controllers::Controller;
use crate::http_server::parse::parse_http_stream;
use crate::match_request;
use crate::thread_pool::ThreadPool;

const ADDRESS: &str = "127.0.0.1:7878";

pub fn start_http_server(routes: Arc<HashMap<String, Box<dyn Controller + Sync + Send>>>)
{
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind http_server server");
    println!("Server started!");
    println!("Listening on: http://{}", ADDRESS);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let thread_routes = Arc::clone(&routes);

        pool.execute(move || {
            let request = parse_http_stream(&stream);
            // TODO path matching
            let response = thread_routes.get("").unwrap().handle(&request);
            stream.write_all(response.to_response_data_bytes().as_slice()).expect("Failed to send response!");
        });
    }
}

