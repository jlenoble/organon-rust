use proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput, Error, FieldsNamed, Result };

use synex::Extract;

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
                        executable: ::core::option::Option::None,
                        args: ::core::option::Option::Some(::std::vec![]),
                        env: ::core::option::Option::Some(::std::vec![]),
                        current_dir: ::core::option::Option::None,
                    }
                }
            }
        };

    let quote_commandbuilder =
        quote! {
            pub struct CommandBuilder {
                executable: ::core::option::Option<::std::string::String>,
                args: ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                env: ::core::option::Option<::std::vec::Vec<::std::string::String>>,
                current_dir: ::core::option::Option<::std::string::String>,
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
    let fields: &FieldsNamed = input.extract()?;

    let quote_fields = quote_fields(fields)?;
    let quote_methods = quote_methods(fields)?;

    Ok(
        quote! {
            impl CommandBuilder {
                #quote_methods

                pub fn build(&mut self) -> ::std::result::Result<Command, ::std::boxed::Box<dyn ::std::error::Error>> {
                    ::std::result::Result::Ok(Command { #quote_fields })
                }
            }
        }
    )
}
