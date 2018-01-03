extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(FrawComponent, attributes(fraw))]
pub fn component_builder(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_component(&ast);
    gen.parse().unwrap()
}

fn impl_component(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    
    let mut dependencies = Vec::new();
    let properties = extract_properties(&ast);

    println!("{:?}", properties);
    for property in properties.iter() {
        let &(ref k, ref v) = property;
        println!("{} {:?}", k, v);

        if **k == syn::Ident::from("dependency") {
            if let syn::Lit::Str(ref value, _) = **v {
                let dep = syn::Ident::from(value.clone());
                println!("{}", dep);
                dependencies.push(quote! { 
                    dependencies_mapping.insert(
                        #dep::name(), 
                        ::std::boxed::Box::new(#dep::build())
                    ); 
                }); 

                println!("{:?}", dependencies);
            }
        }
    }
    
    // Suppress warning about mut when there's no element
    let declaration_dependencies = if dependencies.len() > 0 {
        quote! { let mut dependencies_mapping }
    } else {
        quote! { let dependencies_mapping }
    };

    // Bug if directly in final "quote!"
    let dependencies_type = quote! { 
        ::std::collections::HashMap<
            String, 
            ::std::boxed::Box<::fraw::component::Component>
    > };

    quote! {
        #[allow(dead_code)]
        impl #name {
            pub fn name() -> String {
                String::from("name")
            }

            pub fn dependencies() -> #dependencies_type {
                #declaration_dependencies: #dependencies_type = ::std::collections::HashMap::new();
                
                #(#dependencies)*

                dependencies_mapping
            }

            pub fn build() -> Self {
                #name{}
            }
        }
    }
}

fn extract_properties(ast: &syn::DeriveInput) -> Vec<(&syn::Ident, &syn::Lit)> {
    use syn::*;
    ast.attrs
        .iter()
        .filter_map(|attr| {
            // Look for all the strum attributes
            if let &Attribute { value: MetaItem::List(ref ident, ref nested), .. } = attr {
                if ident == "fraw" {
                    return Option::Some(nested);
                }
            }

            Option::None
        })
        .flat_map(|prop| prop)
        .filter_map(|prop| {
            // Only look at key value pairs
            if let &NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, ref value)) = prop {
                return Option::Some((ident, value));
            }

            Option::None
        })
        .collect()
}


// @todo
// - attribute : dependency, name
// - give dependencies to macro ?
// - register dependencies to first program
