use html::Tag;
use component::FrawComponent;
use TAG_ALIASES;

#[macro_export]
macro_rules! view { 
    // Opening tag
    ($component:ident, $dom:ident, $nodes:ident (< $start:ident $($tree:tt)*)) => {
        let tag_str = stringify!($start);

        match $crate::TAG_ALIASES.lock().unwrap().get(&tag_str) {
            Some(alias) => $nodes.push($crate::html::Tag::new(alias)),
            None => $nodes.push($crate::html::Tag::new(tag_str))
        }

        view! { $component, $dom, $nodes($($tree)*) }
    };
    // End opening tag
    ($component:ident, $dom:ident, $nodes:ident (> $($tree:tt)*)) => {
        view! { $component, $dom, $nodes($($tree)*) }
    };
    // Closing tag
    ($component:ident, $dom:ident, $nodes:ident (</ $end:ident > $($tree:tt)*)) => {
        let tag_str = stringify!($end);
        $crate::macros::close_tag($component, &mut $nodes, &mut $dom, Some(tag_str));
        
        view! { $component, $dom, $nodes($($tree)*) }
    };
    // Self closing tag
    ($component:ident, $dom:ident, $nodes:ident (/> $($tree:tt)*)) => {
        $crate::macros::close_tag($component, &mut $nodes, &mut $dom, None);
        
        view! { $component, $dom, $nodes($($tree)*) }
    };
    // Text nodes (expr) + closing tag
    ($component:ident, $dom:ident, $nodes:ident ({ $expr:expr } $($tree:tt)*)) => {
        $crate::macros::Expression::process($expr, &mut $nodes);

        view! { $component, $dom, $nodes($($tree)*) }
    };
    // End of the parsing
    ($component:ident, $dom:ident, $nodes:ident ()) => {
        if $dom.len() > 1 {
            panic!("Your view must only contains one root node");
        }

        $dom.pop().unwrap()
    };
    // Init component
    (($component:ident) => {$($tree:tt)*}) => {{
        // Temporary nodes
        let mut nodes = Vec::new();

        // Final DOM
        let mut dom = Vec::new();

        view! { $component, dom, nodes($($tree)*) }
    }};
    // Init component
    ($($tree:tt)*) => {{
        view! { $($tree)* }
    }};
}

#[doc(hidden)]
// When a closing tag is encountered, this method is triggered to modify the "$dom"
pub fn close_tag(component: &FrawComponent, nodes: &mut Vec<Tag>, dom: &mut Vec<Tag>, tag: Option<&'static str>) {
    match nodes.pop() {
       None => panic!("More closing tags than opening tags (component {})", component.name()),
       Some(last_tag) => { 
           let tag_name: &str;

           match tag {
               None => { tag_name = last_tag.tag; },
               Some(tag_str) => {
                   match TAG_ALIASES.lock().unwrap().get(tag_str) {
                       Some(alias) => { tag_name = alias; },
                       None => { tag_name = tag_str; } 
                   }

                   if last_tag.tag != tag_name {
                       panic!("Closing tag '{}' does not match the opening tag '{}'", tag_name, last_tag.tag)
                   }
               }
           }
           

           // Check for component
           match component.dependencies().get(&String::from(tag_name)) {
               Some(component) => {
                   Expression::process(&component.render(), nodes);
               },
               None => {
                   if !nodes.is_empty() {
                       nodes.last_mut().unwrap().add_child(last_tag);
                   } else {
                       dom.push(last_tag);
                   }        
               }
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
