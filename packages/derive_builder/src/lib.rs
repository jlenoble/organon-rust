use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

mod quote_field_and_methods;
mod quote_fields_and_methods;

use quote_fields_and_methods::quote_fields_and_methods;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (fields, methods) = quote_fields_and_methods(&input.data);

    let expanded =
        quote! {
    use color_eyre::{eyre::eyre, Result};

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

    pub struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }

    impl CommandBuilder {
        #methods

        pub fn build(&mut self) -> Result<Command> {
            Ok(Command { #fields })
        }
    }
        };

    proc_macro::TokenStream::from(expanded)
}