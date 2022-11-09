use proc_macro2::TokenStream;
use quote::quote;
use syn::{ DeriveInput, Result };

use super::extract::quote_ident;
use super::extract_where::quote_find_one_named_field;
use super::field_by_name::quote_find_field_by_name;
use super::field_ident::quote_only_one_named_field_ident;
use super::nth_field_ident::{ quote_one_named_field_ident, quote_second_named_field_ident };

pub fn quote_struct_multiple_fields_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote_ident = quote_ident(derive_input)?;
    let quote_one_field_ident = quote_one_named_field_ident(derive_input)?;
    let quote_second_field_ident = quote_second_named_field_ident(derive_input)?;
    let quote_find_one_field = quote_find_one_named_field(derive_input)?;
    let quote_find_field_by_name = quote_find_field_by_name(derive_input)?;

    Ok(
        quote! {
            #quote_ident
            #quote_one_field_ident
            #quote_second_field_ident
            #quote_find_one_field
            #quote_find_field_by_name
        }
    )
}

pub fn quote_struct_single_field_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote_ident = quote_ident(derive_input)?;
    let quote_only_one_field_ident = quote_only_one_named_field_ident(derive_input)?;
    let quote_one_field_ident = quote_one_named_field_ident(derive_input)?;
    let quote_find_one_field = quote_find_one_named_field(derive_input)?;
    let quote_find_field_by_name = quote_find_field_by_name(derive_input)?;

    Ok(
        quote! {
            #quote_ident
            #quote_only_one_field_ident
            #quote_one_field_ident
            #quote_find_one_field
            #quote_find_field_by_name
        }
    )
}

pub fn quote_unit_struct_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote_ident = quote_ident(derive_input)?;

    Ok(quote! {
        #quote_ident
    })
}