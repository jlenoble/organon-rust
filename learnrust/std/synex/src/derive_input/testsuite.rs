use proc_macro2::TokenStream;
use quote::quote;
use syn::{ DeriveInput, Result };

use super::extract::test_struct_name;
use super::field_by_name::test_find_field_by_name;
use super::field_ident::test_single_field_name;
use super::nth_field_ident::{ test_0th_first_last_field_names, test_second_field_name };

pub fn quote_struct_multiple_fields_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote1 = test_struct_name(derive_input)?;
    let quote2 = test_0th_first_last_field_names(derive_input)?;
    let quote3 = test_second_field_name(derive_input)?;
    let quote4 = test_find_field_by_name(derive_input)?;

    Ok(
        quote! {
            #quote1
            #quote2
            #quote3
            #quote4
        }
    )
}

pub fn quote_struct_single_field_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote1 = test_struct_name(derive_input)?;
    let quote2 = test_single_field_name(derive_input)?;
    let quote3 = test_0th_first_last_field_names(derive_input)?;
    let quote4 = test_find_field_by_name(derive_input)?;

    Ok(
        quote! {
            #quote1
            #quote2
            #quote3
            #quote4
        }
    )
}

pub fn quote_unit_struct_tests(derive_input: &DeriveInput) -> Result<TokenStream> {
    let quote1 = test_struct_name(derive_input)?;

    Ok(quote! {
        #quote1
    })
}