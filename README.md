# Fraw
Webassembly front framework (prototype/test)

This is my experiments with Webassembly and Rust (I'm not a Rust developer).
It's a little "component" framework based on the amazing [Koute's stdweb lib](https://github.com/koute/stdweb).

If you're looking for a probably more active, more functional webassembly framework, in the spirit of Elm feel free to take a look at [yew](https://github.com/DenisKolodin/yew).

# How does it work for now

/!\ It could change, a lot of features are missing

```rust
#[macro_use]
extern crate fraw;

#[macro_use]
extern crate fraw_component_derive;

use fraw::Fraw;
use fraw::component::Component;
use fraw::html::Tag;

pub mod components;

#[derive(Default)]
#[derive(FrawComponent)]
#[fraw_selector = "mycmp"]
#[fraw_dependency = "::components::MySecondCmp"]
#[fraw_dependency(dep = "::components::MyThirdCmp", selector = "mycustomthirdcmp")]
struct MyCmp {}
impl Component for MyCmp {
    fn render(&self) -> Tag {
        view! { (self) => {
            <div>
                <p>{ "View MyCmp" }</p>
                <my_test class={ self.test().as_str() } id="myid"></my_test>
                <mysecondcmp />    
                <mycustomthirdcmp/>   
                <test_alias_cmp/> 
            </div>
        } }
    }
}

impl MyCmp {
    pub fn test(&self) -> String {
        String::from("test_class")
    } 
}

fn main() {
    Fraw::register_alias("my_test", "my-test");
    Fraw::register_alias("test_alias_cmp", "mysecondcmp");

    let fraw = Fraw::init("body", Box::new(MyCmp{}));

    fraw.run();
}
```

As you can see, a component is a struct that have the attribute `#[derive(FrawComponent)]`.

Once it's done you have to :
- Implement the `Default` trait (used for component instanciation)
- Declare you're dom selector with `#[fraw_selector = <selector>]`
- Implement the `render(&self) -> Tag` method, that returns a template using the `view!` macro. (you always need the `(self) =>` in order to get your component's dependencies if you have any)

If you want to use another components in your, you have to declare the dependency as in the exemple :
- `#[fraw_dependency = "::full::namespace::component"]` (note that you have to give the complete path to your component)
- `#[fraw_dependency(dep = "::full::namespace::component", selector="custom_dom_selector")]` to customize the selector in your view

Then, in your main, just call  
```rust
let fraw = Fraw::init("body", Box::new(YourRootCmp{}));

fraw.run();
```

## Note

In rust, you can't use `-` as identifiers, so tags with `-` are not valid. To be able to use existing tags (like from ionic), you can use `Fraw::register_alias("alias", "original-tag");`. In your code you will use `alias` but it's `original-tag` that will be in your DOM !

## Thank you

Thank you for reading, remember, everything is just experiments, I'm not a rust developer, nothing stable and a lot of features missing.
