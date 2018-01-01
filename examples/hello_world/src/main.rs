#[macro_use]
extern crate fraw;

#[macro_use]
extern crate fraw_component_derive;

use fraw::init_program;
use fraw::component::Component;
use fraw::html::Tag;
use fraw::COMPONENTS;

pub mod components;

#[derive(FrawComponent)]
struct MyCmp {}
impl Component for MyCmp {
    fn render(&self) -> Tag {
        view! {
            <div>{ "View MyCmp" }</div>
        }
    }
}

fn main() {
    COMPONENTS.lock().unwrap().insert(String::from("mycmp"), Box::new(MyCmp::build()));
    COMPONENTS.lock().unwrap().insert(String::from("mysecondcmp"), Box::new(components::MySecondCmp::build()));

    init_program("body", view! { 
        <div>
            <p>{ "test" }</p>
            <p>
                <a>{ "ok ok" }</a>
            </p>
            <p></p>
            <div>
                <mycmp></mycmp>
                <mysecondcmp></mysecondcmp>
            </div>
        </div> 
    });
}
