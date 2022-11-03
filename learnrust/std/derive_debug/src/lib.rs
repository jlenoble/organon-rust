use proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ DeriveInput, parse_macro_input };

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote_all(&parse_macro_input!(input as DeriveInput)).into()
}

fn quote_all(_input: &DeriveInput) -> TokenStream {
    quote! {}
}