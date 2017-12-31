extern crate stdweb;

#[macro_use] extern crate lazy_static;

use stdweb::web::document;
use stdweb::web::INode;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod html;
pub mod macros;
pub mod component;

use html::Tag;
use component::Component;

lazy_static! {
    pub static ref COMPONENTS: Mutex<HashMap<String, Box<Component + Send + Sync>>> = Mutex::new(HashMap::new());
}

/// Init everything !
pub fn init_program(selector: &str, root: Tag) {
    stdweb::initialize();

    match document().query_selector(selector) {
        None => panic!("No selector {} found", selector),
        Some(element) => {
            element.append_child(&root.render());
        }
    };

    stdweb::event_loop();
}

