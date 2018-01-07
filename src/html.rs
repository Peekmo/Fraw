use stdweb::web::document;
use stdweb::web::Element;
use stdweb::web::INode;
use std::collections::HashMap;
use std::fmt;

/// HTML Tag representation
pub struct Tag {
    pub tag: &'static str,
    children: Vec<Box<Tag>>,
    attributes: HashMap<String, String>,
    listeners: HashMap<String, Box<Fn()>>,
    inner_html: Option<String>
}

/// HTML Tag functions ! OMG magic is here !
impl Tag {
    /// Constructor
    pub fn new(tag: &'static str) -> Self {
        Tag { 
            tag: tag, 
            children: Vec::new(), 
            attributes: HashMap::new(),
            listeners: HashMap::new(),
            inner_html: None 
        }
    }

    /// Render the tag (and its children)
    pub fn render(&self) -> Element {
        let element = document().create_element(self.tag);

        // @todo Not implemented yet by stdweb
        for (name, value) in self.attributes.iter() {
            js!( @{&element}.setAttribute(@{name.as_str()}, @{value.as_str()}); );
        }

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

    /// Add an HTML attribute
    pub fn add_attribute(&mut self, name: &str, value: &str) {
        self.attributes.insert(String::from(name), String::from(value));
    }

    /// Set the text that will be inside the HTML tag
    pub fn set_inner_html(&mut self, text: &str) {
        if self.children.len() > 0 {
            panic!("Impossible to add expression to a node with children ({})", text);    
        }

        self.inner_html = Some(text.to_string());
    }

    /// Add a tag as a child of the current tag
    pub fn add_child(&mut self, tag: Box<Tag>) {
        if let Some(ref text) = self.inner_html {
            panic!("Impossible to add children to a node with expression ({})", text);    
        }

        self.children.push(tag);
    }

    pub fn add_listener(&mut self, name: &str, listener: Box<Fn()>) {
        self.listeners.insert(String::from(name), listener);
    }
}


