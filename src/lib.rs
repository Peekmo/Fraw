extern crate stdweb;

use stdweb::web::document;
use stdweb::web::INode;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod html;
pub mod macros;
pub mod component;

use html::Tag;
use component::FrawComponent;

/// Init everything !
pub fn init_program(selector: &str, root: Box<FrawComponent>) {
    stdweb::initialize();

    match document().query_selector(selector) {
        None => panic!("No selector {} found", selector),
        Some(element) => {
            element.append_child(&root.render().render());
        }
    };

    stdweb::event_loop();
}

