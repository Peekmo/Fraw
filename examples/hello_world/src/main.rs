#[macro_use]
extern crate fraw;

use fraw::init_program;
use fraw::html::Tag;

fn main() {
    init_program("body", view! { 
        <div>
            <p></p>
            <p>
                <a></a>
            </p>
            <p></p>
        </div> 
    });
}
