extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::collections::HashMap;

#[proc_macro_derive(FrawComponent, attributes(fraw_dependency, fraw_selector))]
pub fn component_builder(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_component(&ast);
    gen.parse().unwrap()
}

/// Implement FrawComponent trait
fn impl_component(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    
    let mut dependencies = Vec::new();
    let component_dependencies = extract_dependencies(&ast);

    // Build the dependencies hashmap
    for (dep, selector) in component_dependencies {
        let dep_ident = syn::Ident::from(dep.clone());
        
        // Custom selector #[fraw_dependency(dep = "xx", selector = "xx")]
        if let Some(custom_selector) = selector {
            dependencies.push(quote! { 
                dependencies_mapping.insert(
                    String::from(#custom_selector), 
                    ::std::boxed::Box::new(#dep_ident::build())
                ); 
            }); 
        // default selector #[fraw_dependency = "xx"]
        } else {
            dependencies.push(quote! { 
                dependencies_mapping.insert(
                    #dep_ident::selector(), 
                    ::std::boxed::Box::new(#dep_ident::build())
                ); 
            });
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
            ::std::boxed::Box<::fraw::component::FrawComponent>
    > };

    // Component default selector
    let component_selector = extract_component_selector(&ast);

    // Component full name
    let component_name = name.as_ref();

    // Generated code
    quote! {
        #[allow(dead_code)]
        impl ::fraw::component::FrawComponent for #name {
            /// Component's default selector
            fn selector() -> String {
                String::from(#component_selector)
            }

            /// Component's dependencies on other components
            fn dependencies(&self) -> #dependencies_type {
                #declaration_dependencies: #dependencies_type = ::std::collections::HashMap::new();
                
                #(#dependencies)*

                dependencies_mapping
            }

            /// Returns component's name
            fn name(&self) -> String {
                String::from(#component_name)    
            }

            /// Build the component
            fn build() -> Self {
                ::std::default::Default::default() 
            }
        }
    }
}

/// Returns the component default selector
fn extract_component_selector(ast: &syn::DeriveInput) -> String {
    use syn::*;
    
    for attr in ast.attrs.iter() {
        match attr {
            // #[fraw_selector = "myselector"]
            &Attribute { value: MetaItem::NameValue(ref ident, ref value), .. } => {
                if let syn::Lit::Str(ref v, _) = *value {
                    if *ident == syn::Ident::from("fraw_selector") {
                        return String::from(v.clone());
                    }    
                }
            }
            _ => {}
        }
    }

    panic!("Unable to find the selector of the component {}", ast.ident);
}

/// Returns the dependencies of the component
fn extract_dependencies(ast: &syn::DeriveInput) -> HashMap<String, Option<String>> {
    use syn::*;
    
    let mut dependencies = HashMap::new();

    for attr in ast.attrs.iter() {
        match attr {
            // Dependencies as #[fraw_dependency(dep = "::mycmp", selector = "mycustomcmp")]
            &Attribute { value: MetaItem::List(ref ident, ref nested), .. } => {
                if ident == "fraw_dependency" {
                    let mut selector = None;
                    let mut identifier = String::from("");

                    // For each element in fraw_dependency
                    for nested_meta_item in nested.iter() {
                        if let &NestedMetaItem::MetaItem(MetaItem::NameValue(ref ident, ref value)) = nested_meta_item {
                            // dep()
                            if *ident == syn::Ident::from("dep")  { 
                                if let syn::Lit::Str(ref v, _) = *value {
                                    identifier = String::from(v.clone());
                                }
                            // selector()
                            } else if *ident == syn::Ident::from("selector") { 
                                if let syn::Lit::Str(ref v, _) = *value {
                                    selector = Some(String::from(v.clone()));
                                }
                            // everything else
                            } else {
                                panic!("Unknow attribute {} for fraw_dependency", ident);
                            }
                        }
                    }

                    dependencies.insert(identifier, selector);
                }
            },
            // Dependencies as #[fraw_dependency = "::mycmp"]
            &Attribute { value: MetaItem::NameValue(ref ident, ref value), .. } => {
                if let syn::Lit::Str(ref v, _) = *value {
                    if *ident == syn::Ident::from("fraw_dependency") {
                        dependencies.insert(String::from(v.clone()), None);
                    }    
                }
            }
            _ => {}
        }
    }

    dependencies
}

