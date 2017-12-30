#[macro_export]
macro_rules! view {
    (< $start:ident >) => {
        $crate::html::Tag::new(stringify!($start), None);
    }
}
