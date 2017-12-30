extern crate fraw;

use fraw::init_program;
use fraw::html::Tag;

fn main() {
    let mut children = Vec::new();
    children.push(Box::new(Tag::new("h1", None)));
    init_program("body", Tag::new("p", Some(children)));
}
