use crate::controllers::Controller;
use crate::http_server::request::Request;
use crate::http_server::response::file_response::FileResponse;
use crate::http_server::response::Response;

pub struct FileController {

}

impl FileController {
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller for FileController {
    fn handle(&self, request: &Request) -> Box<dyn Response> {
        println!("path: {}", &request.path);

        for part in request.path.split('/').filter(|x| !x.is_empty()) {
            println!("\"{}\"", part)
        }

        let file_path = format!("public{}", &request.path);
        println!("Fil path: {}", &file_path);

        Box::new(FileResponse::ok(file_path))
    }
}
