#[macro_use]
extern crate fraw;

#[macro_use]
extern crate fraw_component_derive;

use fraw::Fraw;
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
                <my_test checked={ true } class={ self.test().as_str() } id="myid"></my_test>
                <mysecondcmp />    
                <mycustomthirdcmp/>   
                <test_alias_cmp/> 
            </div>
        } }
    }
}

impl MyCmp {
    pub fn test(&self) -> String {
        String::from("test_class")
    } 
}

fn main() {
    Fraw::register_alias("my_test", "my-test");
    Fraw::register_alias("test_alias_cmp", "mysecondcmp");

    let fraw = Fraw::init("body", Box::new(MyCmp{}));

    fraw.run();
}
