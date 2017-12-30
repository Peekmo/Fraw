#[macro_use]
extern crate fraw;

use fraw::init_program;

fn main() {
    init_program("body", view! { 
        <div>
            <p>{ "test" }</p>
            <p>
                <a>{ "ok ok" }</a>
            </p>
            <p></p>
            <div>{ view! {
                <div>{ "View inside a view" }</div>
            } }</div>
        </div> 
    });
}
