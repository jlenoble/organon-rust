use proc_macro2::TokenStream;
use quote::quote;
use syn::{ Data, Fields };

use super::quote_field_and_methods::quote_field_and_methods;

pub fn quote_fields_and_methods(data: &Data) -> (TokenStream, TokenStream) {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let (each_field, each_method): (
                        Vec<TokenStream>,
                        Vec<TokenStream>,
                    ) = fields.named
                        .iter()
                        .map(|field| { quote_field_and_methods(field) })
                        .unzip();

                    (
                        quote! {
                        #(#each_field ,)*
                    },
                        quote! { #(#each_method)* },
                    )
                }
                _ => {
                    panic!("Fields in Struct must be named");
                }
            }
        }
        _ => {
            panic!("Build can only be derived for Structs, not Enums nor Unions");
        }
    }
}