extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{ DeriveInput, Error, parse_macro_input };

use super::derive_input;

pub fn quote_struct_multiple_fields_tests(input: TokenStream) -> TokenStream {
    let quote_all = derive_input
        ::quote_struct_multiple_fields_tests(&parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(Error::into_compile_error);

    quote_all.into()
}

pub fn quote_struct_single_field_tests(input: TokenStream) -> TokenStream {
    let quote_all = derive_input
        ::quote_struct_single_field_tests(&parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(Error::into_compile_error);

    quote_all.into()
}

pub fn quote_unit_struct_tests(input: TokenStream) -> TokenStream {
    let quote_all = derive_input
        ::quote_unit_struct_tests(&parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(Error::into_compile_error);

    quote_all.into()
}