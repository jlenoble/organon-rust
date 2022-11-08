use proc_macro2::{ Ident, TokenStream };
use quote::quote;
use syn::{ Field, FieldsNamed, Result, TypePath };

use synex::{ Extract, ExtractIter };

pub fn quote_fields(fields: &FieldsNamed) -> Result<TokenStream> {
    let mut quote_each_field: Vec<TokenStream> = Vec::new();

    for field in fields.extract_iter()? {
        quote_each_field.push(quote_field(field)?);
    }

    Ok(quote! { #(#quote_each_field,)* })
}

fn quote_field(field: &Field) -> Result<TokenStream> {
    let field_name: &Ident = field.extract()?;

    let typepath: &TypePath = field.extract()?;
    let type_name: &Ident = typepath.extract()?;

    if type_name == "Option" {
        Ok(
            quote! {
                #field_name : match &self.#field_name {
                    ::core::option::Option::Some(#field_name) => { ::core::option::Option::Some(#field_name.clone()) }
                    ::core::option::Option::None => { ::core::option::Option::None }
                }
            }
        )
    } else {
        let panic_msg = format!("a setter for mandatory field `{}` was never called", field_name);

        Ok(
            quote! {
                #field_name : match &self.#field_name {
                    ::core::option::Option::Some(#field_name) => { #field_name.clone() }
                    ::core::option::Option::None => {
                        panic!(#panic_msg);
                    }
                }
            }
        )
    }
}