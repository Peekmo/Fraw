#[macro_use]
extern crate fraw;

#[macro_use]
extern crate fraw_component_derive;

use fraw::init_program;
use fraw::component::Component;
use fraw::html::Tag;

pub mod components;

#[derive(FrawComponent)]
#[fraw(dependency = "::components::MySecondCmp")]
struct MyCmp {}
impl Component for MyCmp {
    fn render(&self) -> Tag {
        view! { (self) => {
            <div>
                <p>{ "View MyCmp" }</p>
                <mysecondcmp></mysecondcmp>    
            </div>
        } }
    }
}

fn main() {
    init_program("body", Box::new(MyCmp{}));
}
