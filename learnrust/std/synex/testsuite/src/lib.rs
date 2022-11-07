//! Helper crate for testing crate `synex`
//!
//! `synex` is designed to be used in the context of procedural macros. This makes really difficult
//! testing its usage because a crate can either export procedural macros or traits, but not both.
//! So `synex`, exporting traits and trait implementations, can't define any procedural macros
//! and so no code snipets can be written within `synex`: can't test unless we define
//! an auxiliary package implementing proc macros calling `synex` traits/methods/macros to be tested
//! Still we can quote all tests within `synex`, we just can't run them.

use proc_macro::TokenStream;
use synex::{
    quote_struct_multiple_fields_tests,
    quote_struct_single_field_tests,
    quote_unit_struct_tests,
};

#[proc_macro_derive(ExpectStructMultipleFields)]
pub fn struct_multiple_fields_tests(input: TokenStream) -> TokenStream {
    quote_struct_multiple_fields_tests(input)
}

#[proc_macro_derive(ExpectStructSingleField)]
pub fn struct_single_field_tests(input: TokenStream) -> TokenStream {
    quote_struct_single_field_tests(input)
}

#[proc_macro_derive(ExpectUnitStruct)]
pub fn unit_struct_tests(input: TokenStream) -> TokenStream {
    quote_unit_struct_tests(input)
}