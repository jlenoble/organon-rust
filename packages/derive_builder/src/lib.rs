use proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput, Error, Fields, Result };

use impl_extract_for_syn::Extract;

use crate::{ quote_fields::quote_fields, quote_methods::quote_methods };

mod quote_fields;
mod quote_methods;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote_all(&parse_macro_input!(input as DeriveInput)).into()
}

fn quote_all(input: &DeriveInput) -> TokenStream {
    let quote_impl_command =
        quote! {
            impl Command {
                pub fn builder() -> CommandBuilder {
                    CommandBuilder {
                        executable: None,
                        args: Some(vec![]),
                        env: Some(vec![]),
                        current_dir: None,
                    }
                }
            }
        };

    let quote_commandbuilder =
        quote! {
            pub struct CommandBuilder {
                executable: Option<String>,
                args: Option<Vec<String>>,
                env: Option<Vec<String>>,
                current_dir: Option<String>,
            }
        };

    let quote_impl_commandbuilder = quote_impl_commandbuilder(input).unwrap_or_else(
        Error::into_compile_error
    );

    quote! {
        #quote_impl_command

        #quote_commandbuilder

        #quote_impl_commandbuilder
    }
}

fn quote_impl_commandbuilder(input: &DeriveInput) -> Result<TokenStream> {
    let fields: &Fields = input.extract()?;

    let quote_fields = quote_fields(fields)?;
    let quote_methods = quote_methods(fields)?;

    Ok(
        quote! {
            impl CommandBuilder {
                #quote_methods

                pub fn build(&mut self) -> Result<Command, Box<dyn ::std::error::Error>> {
                    Ok(Command { #quote_fields })
                }
            }
        }
    )
}