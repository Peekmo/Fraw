#[macro_use]
extern crate fraw;

use fraw::init_program;
use fraw::component::Component;
use fraw::html::Tag;
use fraw::COMPONENTS;

struct MyCmp {}
impl Component for MyCmp {
    fn render(&self) -> Tag {
        view! {
            <div>{ "View MyCmp" }</div>
        }
    }
}

fn main() {
    COMPONENTS.lock().unwrap().insert(String::from("mycmp"), Box::new(MyCmp{}));

    init_program("body", view! { 
        <div>
            <p>{ "test" }</p>
            <p>
                <a>{ "ok ok" }</a>
            </p>
            <p></p>
            <div>
                <mycmp></mycmp>
            </div>
        </div> 
    });
}
