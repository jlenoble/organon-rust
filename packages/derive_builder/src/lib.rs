use proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ parse_macro_input, DeriveInput, Data, Fields, Field, Type, TypePath, Path, PathSegment };

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = quote_fields(&input.data);

    let expanded =
        quote! {
    use color_eyre::{eyre::eyre, Result};

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

    pub struct CommandBuilder {
        executable: Option<String>,
        args: Option<Vec<String>>,
        env: Option<Vec<String>>,
        current_dir: Option<String>,
    }

    impl CommandBuilder {
        fn executable(&mut self, executable: String) -> &mut Self {
            self.executable = Some(executable);
            self
        }

        fn args(&mut self, args: Vec<String>) -> &mut Self {
            self.args = Some(args);
            self
        }

        fn env(&mut self, env: Vec<String>) -> &mut Self {
            self.env = Some(env);
            self
        }

        fn current_dir(&mut self, current_dir: String) -> &mut Self {
            self.current_dir = Some(current_dir);
            self
        }

        pub fn build(&mut self) -> Result<Command> {
            Ok(Command { #fields })
        }
    }
        };

    proc_macro::TokenStream::from(expanded)
}

fn quote_field(field: &Field) -> TokenStream {
    let (name, segments) = if
        let Field {
            ref ident,
            ty: Type::Path(TypePath { path: Path { ref segments, .. }, qself: None }),
            ..
        } = field
    {
        (ident, segments)
    } else {
        panic!("Not a field");
    };

    let name = match name {
        Some(name) => name,
        None => panic!("Found unnamed field"),
    };

    let is_optional = if let Some(&PathSegment { ref ident, .. }) = segments.first() {
        ident == "Option"
    } else {
        false
    };

    if is_optional {
        quote! { 
            #name : match &self.#name {
                Some(#name) => { Some(#name.clone()) }
                None => { None }
            } 
        }
    } else {
        quote! {
            #name : match &self.#name {
                Some(#name) => { #name.clone() }
                None => {
                    return Err(eyre!("setter was never called on your builder"));
                }
            }
        }
    }
}

fn quote_fields(data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let each_field = fields.named.iter().map(|field| { quote_field(field) });

                    quote! {
                        #(#each_field ,)*
                    }
                }
                _ => {
                    panic!("Fields in Struct must be named");
                }
            }
        }
        _ => {
            panic!("Build can only be derived for Structs, not Enums nor Unions");
        }
    }
}