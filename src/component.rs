use html::Tag;

pub trait Component {
    fn render(&self) -> Tag;
}
