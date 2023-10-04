use std::io::{Write};
use std::net::{TcpListener};
use std::sync::Arc;
use crate::controllers::{Controller};
use crate::controllers::controller_list::ControllerList;
use crate::controllers::not_found_controller::NotFoundController;
use crate::http_server::parse::parse_http_stream;
use crate::http_server::response::{Response};
use crate::thread_pool::ThreadPool;

const ADDRESS: &str = "127.0.0.1:7878";

pub fn start_http_server(routes: Arc<ControllerList>)
{
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind http_server server");
    println!("Server started!");
    println!("Listening on: http://{}", ADDRESS);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let thread_routes = Arc::clone(&routes);

        pool.execute(move || {
            let mut request = parse_http_stream(&stream);

            let response: Box<dyn Response> = match thread_routes.get(&request.method, &request.path) {
                None => NotFoundController::new().handle(&request),
                Some((parameters, controller)) => {
                    request.parameters = parameters;
                    controller.handle(&request)
                },
            };

            stream.write_all(response.to_response_data_bytes().as_slice()).expect("Failed to send response!");
        });
    }
}
