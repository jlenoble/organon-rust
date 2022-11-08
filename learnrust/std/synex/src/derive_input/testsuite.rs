use proc_macro2::TokenStream;
use quote::quote;
use syn::{ DeriveInput, Result };

use super::extract::{ quote_ident, quote_only_one_named_field_ident };

pub fn quote_struct_multiple_fields_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote_ident = quote_ident(derive_input)?;

    Ok(quote! {
        #quote_ident
    })
}

pub fn quote_struct_single_field_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote_ident = quote_ident(derive_input)?;
    let quote_only_one_field_ident = quote_only_one_named_field_ident(derive_input)?;

    Ok(quote! {
        #quote_ident
        #quote_only_one_field_ident
    })
}

pub fn quote_unit_struct_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote_ident = quote_ident(derive_input)?;

    Ok(quote! {
        #quote_ident
    })
}