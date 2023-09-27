use std::collections::HashMap;
use crate::controllers::Controller;
use crate::http_server::method::Method;

pub struct ControllerList {
    method_map: HashMap<Method, Vec<Box<dyn Controller + Sync + Send>>>
}

impl ControllerList {
    pub fn from(controllers: Vec<(Method, Box<dyn Controller + Sync + Send>)>) -> Self {
        let mut controller_list = Self {
            method_map: HashMap::new()
        };

        for (method, controller) in controllers {
            controller_list.add(method, controller);
        }

        controller_list
    }

    pub fn add(&mut self, method: Method, controller: Box<dyn Controller + Sync + Send>) {
        if let Some(controllers) = self.method_map.get_mut(&method) {
            controllers.push(controller);
        } else {
            self.method_map.insert(method, vec![
                controller,
            ]);
        }
    }

    pub fn get(&self, method: &Method, path: &String) -> Option<&Box<dyn Controller + Sync + Send>> {
        let controllers = match self.method_map.get(method) {
            None => return None,
            Some(controllers) => controllers
        };

        let path_sections = path.trim_start_matches("/").split("/").collect::<Vec<&str>>();

        for controller in controllers {
            if let Some(matching_controller) = controller_matches(&path_sections, controller) {
                return Some(matching_controller);
            }
        }

        None
    }

}

fn controller_matches<'a>(path_sections: &Vec<&str>, controller: &'a Box<dyn Controller + Send + Sync>) -> Option<&'a Box<dyn Controller + Send + Sync>> {
    let controller_path = controller.path();
    let controller_sections = controller_path.trim_start_matches("/").split("/").collect::<Vec<&str>>();

    if path_sections.len() != controller_sections.len() {
        return None;
    }

    for (index, path_section) in path_sections.iter().enumerate() {
        let controller_section = controller_sections[index];

        // TODO: parameter matching
        if *controller_section != **path_section {
            return None;
        }
    }

    Some(controller)
}

