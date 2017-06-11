extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use syn::{Body, VariantData, MacroInput};

#[proc_macro_derive(NewtypeProxy)]
pub fn newtype_proxy(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_newtype_proxy(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_newtype_proxy(ast: &MacroInput) -> quote::Tokens {
    match ast.body {
        Body::Struct(VariantData::Tuple(ref fields)) => {
            if fields.len() == 1 {
                let name = &ast.ident;
                let (impl_vars, type_vars, where_clause) = ast.generics.split_for_impl();
                let inner_type = &fields.iter().next().unwrap().ty;
                quote! {
                    impl#impl_vars ::std::convert::From<#inner_type> for #name#type_vars #where_clause {
                        fn from(a: #inner_type) -> #name#type_vars {
                            #name(a)
                        }
                    }

                    impl#impl_vars ::std::ops::Deref for #name#type_vars #where_clause {
                        type Target = #inner_type;
                        fn deref(&self) -> &#inner_type {
                            &self.0
                        }
                    }
                }
            } else {
                panic!("derive(NewtypeProxy) only supports single value newtypes.")
            }
        },
        _ => panic!("derive(NewtypeProxy) only supports single value newtypes.")
    }
}
