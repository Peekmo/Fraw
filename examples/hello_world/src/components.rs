use fraw::component::Component;
use fraw::html::Tag;

#[derive(Default)]
#[derive(FrawComponent)]
#[fraw_selector = "mysecondcmp"]
pub struct MySecondCmp {}
impl Component for MySecondCmp {
    fn render(&self) -> Tag {
        view! { (self) => {
            <div>{ "View MySecondCmp" }</div>
        } }  
    }
}

#[derive(Default)]
#[derive(FrawComponent)]
#[fraw_selector = "mythirdcmp"]
#[fraw_dependency = "::components::MySecondCmp"]
pub struct MyThirdCmp {}
impl Component for MyThirdCmp {
    fn render(&self) -> Tag {
        view! { (self) => {
            <div>
                <p>{ "View MyThirdCmp" }</p>
                <mysecondcmp/>
            </div> 
        } }  
    }
}


