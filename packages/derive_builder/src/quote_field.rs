use proc_macro2::{ TokenStream, Ident };
use quote::quote;

pub fn quote_field(field_name: &Ident, is_optional: bool) -> TokenStream {
    if is_optional {
        quote! { 
            #field_name : match &self.#field_name {
                Some(#field_name) => { Some(#field_name.clone()) }
                None => { None }
            } 
        }
    } else {
        quote! {
            #field_name : match &self.#field_name {
                Some(#field_name) => { #field_name.clone() }
                None => {
                    return Err(eyre!("setter was never called on your builder"));
                }
            }
        }
    }
}