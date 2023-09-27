use crate::http_server::request::Request;
use crate::http_server::response::Response;

pub mod file_controller;
pub mod api_controller;
pub mod controller_list;
pub mod not_found_controller;
pub mod get_character_controller;
pub mod character_index_controller;

pub trait Controller {
    fn handle(&self, request: &Request) -> Box<dyn Response>;
    fn path(&self) -> String;
}

pub type SyncController = dyn Controller + Send + Sync;
