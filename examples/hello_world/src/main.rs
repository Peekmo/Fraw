#[macro_use]
extern crate fraw;

#[macro_use]
extern crate fraw_component_derive;

use fraw::init_program;
use fraw::component::Component;
use fraw::html::Tag;

pub mod components;

#[derive(Default)]
#[derive(FrawComponent)]
#[fraw_selector = "mycmp"]
#[fraw_dependency = "::components::MySecondCmp"]
#[fraw_dependency(dep = "::components::MyThirdCmp", selector = "mycustomthirdcmp")]
struct MyCmp {}
impl Component for MyCmp {
    fn render(&self) -> Tag {
        view! { (self) => {
            <div>
                <p>{ "View MyCmp" }</p>
                <mysecondcmp></mysecondcmp>    
                <mycustomthirdcmp></mycustomthirdcmp>    
                <mythirdcmp></mythirdcmp>    
            </div>
        } }
    }
}

fn main() {
    init_program("body", Box::new(MyCmp{}));
}
