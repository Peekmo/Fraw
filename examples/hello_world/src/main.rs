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
struct MyCmp {
    id: i32,
    name: String
}
impl Component for MyCmp {
    fn render(&self) -> Tag {
        view! { (self) => {
            <div>
                <p>{ "View MyCmp" }</p>
                <my_test 
                    checked={ true } 
                    class={ self.test().as_str() } 
                    id={ self.id.to_string().as_str() }
                    name="test"
                    (click)={ || println!("test") }
                ></my_test>
                <mysecondcmp />    
                <mycustomthirdcmp/>   
                <test_alias_cmp/> 
            </div>
        } }
    }
}

impl MyCmp {
    pub fn test(&self) -> String { 
        self.name.clone()
    } 

    pub fn change_name(&mut self) {
        fraw_state! { (self) => {
            name: String::from("test_name"),
            id: 1000
        } } 
    }

    pub fn onclick() {
        println!("ok");
        /*fraw_state! { (self) => {
            name: String::from("clicked"),
            id: 9999
        } };

        self.render();*/
    }
}

fn main() {
    Fraw::register_alias("my_test", "my-test");
    Fraw::register_alias("test_alias_cmp", "mysecondcmp");

    let mut cmp = Box::new(MyCmp{id: 0, name: String::from("")});
    cmp.change_name();
    let fraw = Fraw::init("body", cmp);

    fraw.run();
}
