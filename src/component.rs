use html::Tag;
use std::collections::HashMap;

pub trait Component {
    fn render(&self) -> Tag;
}

pub trait FrawComponent: Component {
    fn dependencies(&self) -> HashMap<String, Box<FrawComponent>>;
    fn name(&self) -> String;
    fn selector() -> String where Self: Sized;
    fn build() -> Self where Self: Sized;
}
