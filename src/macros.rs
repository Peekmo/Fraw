#[macro_export]
macro_rules! view {
    ($dom:ident (< $start:ident > $($tree:tt)*)) => {
        let tag = $crate::html::Tag::new(stringify!($start));
        $dom.push(tag);
        view! { $dom($($tree)*) }
    };
    ($dom:ident (</ $end:ident > $($tree:tt)*)) => {
        let last_tag = $dom.pop().unwrap();
        if !$dom.is_empty() {
            let mut parent = $dom.pop().unwrap();
            parent.add_child(last_tag);
            $dom.push(parent);
        } else {
            $dom.push(last_tag);
        }

        view! { $dom($($tree)*) }
    };
    ($dom:ident ()) => {
        $dom.pop().unwrap()
    };
    ($($tree:tt)*) => {{
        let mut dom = Vec::new();
        view! { dom($($tree)*) }
    }}
}
