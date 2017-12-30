use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;

#[derive(Clone)]
/// HTML Tag representation
pub struct Tag {
    pub tag: &'static str,
    children: Vec<Tag>,
    inner_html: Option<String>
}

/// HTML Tag functions ! OMG magic is here !
impl Tag {
    /// Constructor
    pub fn new(tag: &'static str) -> Self {
        Tag { 
            tag: tag, 
            children: Vec::new(), 
            inner_html: None 
        }
    }

    /// Render the tag (and its children)
    pub fn render(&self) -> Element {
        let element = document().create_element(self.tag);

        match self.inner_html {
            Some(ref text) => element.set_text_content(text.as_str()),
            None => {
                for child in self.children.iter() {
                    element.append_child(&child.render());
                }        
            }
        }

        element
    }

    /// Set the text that will be inside the HTML tag
    pub fn set_inner_html(&mut self, text: &str) {
        if self.children.len() > 0 {
            panic!("Impossible to add expression to a node with children ({})", text);    
        }

        self.inner_html = Some(text.to_string());
    }

    /// Add a tag as a child of the current tag
    pub fn add_child(&mut self, tag: Tag) {
        if let Some(ref text) = self.inner_html {
            panic!("Impossible to add children to a node with expression ({})", text);    
        }

        self.children.push(tag);
    }
}


