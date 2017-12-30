use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;

/// HTML Tag
pub struct Tag {
    tag: &'static str,
    children: Vec<Tag>
}

/// HTML Tag functions
impl Tag {
    pub fn new(tag: &'static str) -> Self {
        Tag { tag: tag, children: Vec::new() }
    }

    pub fn render(self) -> Element {
        let element = document().create_element(self.tag);

        for child in self.children {
            element.append_child(&child.render());
        }

        return element
    }

    pub fn add_child(&mut self, tag: Tag) {
        self.children.push(tag);
    }
}


