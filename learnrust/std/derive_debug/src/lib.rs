use proc_macro;
use proc_macro2::{ TokenStream, Ident };
use quote::quote;
use syn::{ DeriveInput, Error, parse_macro_input, Result };

#[proc_macro_derive(CustomDebug)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote_all(&parse_macro_input!(input as DeriveInput)).into()
}

fn quote_all(input: &DeriveInput) -> TokenStream {
    let quote_impl_debug = quote_impl_debug(input).unwrap_or_else(Error::into_compile_error);

    quote! {
        #quote_impl_debug
    }
}

fn quote_impl_debug(input: &DeriveInput) -> Result<TokenStream> {
    let struct_name: &Ident = &input.ident;
    let struct_name_as_string = struct_name.to_string();

    Ok(
        quote! {
            impl ::std::fmt::Debug for #struct_name {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct(#struct_name_as_string)
                        .field("name", &self.name)
                        .field("bitmask", &self.bitmask)
                        .finish()
                }
            }
        }
    )
}