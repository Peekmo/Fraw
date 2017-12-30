use html::Tag;

#[macro_export]
macro_rules! view { 
    // Opening tag
    ($dom:ident, $nodes:ident (< $start:ident > $($tree:tt)*)) => {
        let tag = $crate::html::Tag::new(stringify!($start));
        $nodes.push(tag);

        view! { $dom, $nodes($($tree)*) }
    };
    // Closing tag
    ($dom:ident, $nodes:ident (</ $end:ident > $($tree:tt)*)) => {
        $crate::macros::close_tag(&mut $nodes, &mut $dom, stringify!($end));    

        view! { $dom, $nodes($($tree)*) }
    };
    // Text nodes (expr)
    ($dom:ident, $nodes:ident ({ $expr:expr } $($tree:tt)*)) => {
        $crate::macros::add_text(&mut $nodes, $expr);

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

#[doc(hidden)]
// Adds text to a node
pub fn add_text(nodes: &mut Vec<Tag>, expr: &'static str) {
    match nodes.pop() {
        None => panic!("Can't write expression outside a tag"),
        Some(mut last_tag) => {
            last_tag.set_inner_html(expr);
            nodes.push(last_tag);
        }
    }
}
