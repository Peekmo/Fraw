#[macro_use]
extern crate stdweb;

use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;

pub mod html;

use html::Tag;

/// Init everything !
pub fn init_program(selector: &str, root: Tag) {
    stdweb::initialize();

    match document().query_selector(selector) {
        None => panic!("No selector {} found", selector),
        Some(element) => element.append_child(&root.render()),
    };

    stdweb::event_loop();
}

