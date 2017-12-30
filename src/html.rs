use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;

/// HTML Tag
pub struct Tag {
    tag: &'static str,
    children: Option<Vec<Box<Tag>>>
}

/// HTML Tag functions
impl Tag {
    pub fn new(tag: &'static str, children: Option<Vec<Box<Tag>>>) -> Self {
        Tag { tag, children }
    }

    pub fn render(self) -> Element {
        let element = document().create_element(self.tag);

        match self.children {
            None => element.set_text_content("test"),
            Some(children) => {
                for child in children {
                    element.append_child(&child.render());
                }
            }
        };
        

        return element
    }
}


