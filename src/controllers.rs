use crate::http_server::request::Request;
use crate::http_server::response::Response;

pub mod file_controller;
pub mod api_controller;

pub trait Controller {
    fn handle(&self, request: &Request) -> Box<dyn Response>;
}
