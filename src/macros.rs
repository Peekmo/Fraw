use html::Tag;

#[macro_export]
macro_rules! view { 
    // Opening tag
    ($dom:ident, $nodes:ident (< $start:ident > $($tree:tt)*)) => {
        let tag_str = stringify!($start);
        let tag = $crate::html::Tag::new(tag_str);
        $nodes.push(tag);

        view! { $dom, $nodes($($tree)*) }
    };
    // Closing tag
    ($dom:ident, $nodes:ident (</ $end:ident > $($tree:tt)*)) => {
        let tag_str = stringify!($end);

        if let Some(component) = $crate::COMPONENTS.lock().unwrap().get(&String::from(tag_str)) {
            $nodes.pop(); // Remove component markup
            $crate::macros::Expression::process(&component.render(), &mut $nodes);
        } else {
            $crate::macros::close_tag(&mut $nodes, &mut $dom, tag_str);    
        }

        view! { $dom, $nodes($($tree)*) }
    };
    // Text nodes (expr)
    ($dom:ident, $nodes:ident ({ $expr:expr } $($tree:tt)*)) => {
        $crate::macros::Expression::process($expr, &mut $nodes);

        view! { $dom, $nodes($($tree)*) }
    };
    // End of the parsing
    ($dom:ident, $nodes:ident ()) => {
        if $dom.len() > 1 {
            panic!("Your view must only contains one root node");
        }

        $dom.pop().unwrap()
    };
    // Init
    ($($tree:tt)*) => {{
        // Temporary nodes
        let mut nodes = Vec::new();

        // Final DOM
        let mut dom = Vec::new();

        view! { dom, nodes($($tree)*) }
    }};
}

#[doc(hidden)]
// When a closing tag is encountered, this method is triggered to modify the "$dom"
pub fn close_tag(nodes: &mut Vec<Tag>, dom: &mut Vec<Tag>, tag: &'static str) {
    match nodes.pop() {
       None => panic!("More closing tags than opening tags '{}'", tag),
       Some(last_tag) => {
           if last_tag.tag != tag {
               panic!("Closing tag '{}' does not match the opening tag '{}'", tag, last_tag.tag)
           }

           if !nodes.is_empty() {
               nodes.last_mut().unwrap().add_child(last_tag);
           } else {
               dom.push(last_tag);
           }
       }
    }
}

/// Expression parsed in template
pub trait Expression {
    /// Process the expression
    fn process(&self, nodes: &mut Vec<Tag>);
}

/// Text node
impl Expression for str {
    fn process(&self, nodes: &mut Vec<Tag>) {
        match nodes.pop() {
            None => panic!("Can't write expression outside a tag"),
            Some(mut last_tag) => {
                last_tag.set_inner_html(self);
                nodes.push(last_tag);
            }
        }
    }
}

/// Subtemplate
impl Expression for Tag {
    fn process(&self, nodes: &mut Vec<Tag>) {
        match nodes.pop() {
            None => panic!("Can't write expression outside a tag"),
            Some(mut last_tag) => {
                last_tag.add_child(self.clone());
                nodes.push(last_tag);
            }
        }
    }
}
