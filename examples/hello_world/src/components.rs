use fraw::component::Component;
use fraw::html::Tag;

#[derive(FrawComponent)]
pub struct MySecondCmp {}
impl Component for MySecondCmp {
    fn render(&self) -> Tag {
        view! {
            <div>{ "View MySecondCmp" }</div>
        }
    }
}
