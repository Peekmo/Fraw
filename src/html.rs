use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;

/// HTML Tag
pub struct Tag {
    tag: &'static str
}

/// HTML Tag functions
impl Tag {
    pub fn new(tag: &'static str) -> Self {
        Tag { tag }
    }

    pub fn render(self) -> Element {
        let element = document().create_element(self.tag);
        element.set_text_content("test");
        
        return element
    }
}


