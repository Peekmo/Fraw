extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
#[macro_use] extern crate lazy_static;

use proc_macro::TokenStream;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref COMPONENTS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

#[proc_macro_derive(FrawComponent)]
pub fn component_builder(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();

    let gen = impl_component(&ast);
    gen.parse().unwrap()
}

fn impl_component(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    println!("{}", name);
    quote! {
        impl #name {
            pub fn build() -> Self {
                #name{}
            }
        }
    }
}

