use quote::quote;
use syn::{ parse_macro_input, DeriveInput };

mod quote_field_and_methods;
mod quote_fields_and_methods;
mod quote_field;
mod quote_methods;

mod attribute_as_path_with_name_value;
mod field_as_named_field_with_attributes_and_type;
mod nested_as_namevalue;
mod nested_meta_as_single_nested;
mod path_as_ident;
mod path_segments_as_ident;
mod path_segments_as_segment;
mod typepath_as_ident;

mod errors;

use quote_fields_and_methods::quote_fields_and_methods;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (fields, methods) = quote_fields_and_methods(&input.data).unwrap();

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