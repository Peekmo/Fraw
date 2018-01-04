extern crate stdweb;

#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::sync::Mutex;
use stdweb::web::document;
use stdweb::web::INode;

pub mod html;
pub mod macros;
pub mod component;

use component::FrawComponent;

lazy_static! {
    pub static ref TAG_ALIASES: Mutex<HashMap<&'static str, &'static str>> = Mutex::new(HashMap::new());
}

/// Fraw program
pub struct Fraw {
    selector: String,
    root: Box<FrawComponent>
}

impl Fraw {
    /// Instantiate the program
    pub fn init(selector: &str, root: Box<FrawComponent>) -> Self {
        Fraw {
            selector: String::from(selector), 
            root: root
        }
    }

    /// Run the program, build the DOM
    pub fn run(&self) {
        stdweb::initialize();

        match document().query_selector(self.selector.as_str()) {
            None => panic!("No selector {} found", self.selector),
            Some(element) => {
                element.append_child(&self.root.render().render());
            }
        };

        stdweb::event_loop();    
    }

    /// Register an alias for a HTML tag
    /// Useful to use non allowed tag names (like `ion-header`) or to globally rename a component selector
    /// Careful, everything is in lowercase
    pub fn register_alias(original: &'static str, alias: &'static str) {
        TAG_ALIASES.lock().unwrap().insert(original, alias);
    }
}

