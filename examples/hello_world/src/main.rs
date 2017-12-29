extern crate fraw;

use fraw::init_program;
use fraw::html::Tag;

fn main() {
    init_program("body", Tag::new("p"));
}
