use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;

/// HTML Tag
pub struct Tag {
    pub tag: &'static str,
    children: Vec<Tag>,
    inner_html: Option<String>
}

/// HTML Tag functions
impl Tag {
    pub fn new(tag: &'static str) -> Self {
        Tag { 
            tag: tag, 
            children: Vec::new(), 
            inner_html: None 
        }
    }

    pub fn render(self) -> Element {
        let element = document().create_element(self.tag);

        match self.inner_html {
            Some(text) => element.set_text_content(text.as_str()),
            None => {
                for child in self.children {
                    element.append_child(&child.render());
                }        
            }
        }

        element
    }

    pub fn set_inner_html(&mut self, text: &'static str) {
        if self.children.len() > 0 {
            panic!("Impossible to add expression to a node with children ({})", text);    
        }

        self.inner_html = Some(text.to_string());
    }

    pub fn add_child(&mut self, tag: Tag) {
        if let Some(ref text) = self.inner_html {
            panic!("Impossible to add children to a node with expression ({})", text);    
        }

        self.children.push(tag);
    }
}


