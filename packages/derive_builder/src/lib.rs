use proc_macro::TokenStream;
use quote::quote;
use syn::{ DeriveInput, parse_macro_input };

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = parse_macro_input!(input as DeriveInput);

    let expanded =
        quote! {
            pub struct CommandBuilder {
                executable: Option<String>,
                args: Option<Vec<String>>,
                env: Option<Vec<String>>,
                current_dir: Option<String>,
            }

            impl Command {
                pub fn builder() -> CommandBuilder {
                    CommandBuilder {
                        executable: None,
                        args: None,
                        env: None,
                        current_dir: None,
                    }
                }
            }
        };

    TokenStream::from(expanded)
}