use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    parse_macro_input,
    Data,
    DataStruct,
    Fields,
    FieldsNamed,
    Type,
    TypePath,
    Field,
    Path,
    PathSegment,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // pub struct Command {
    //     executable: String,
    //     args: Vec<String>,
    //     env: Vec<String>,
    //     current_dir: String,
    // }

    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Build can only be derived for Structs, not Enums nor Unions"),
    };

    let fields = match data {
        DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. } => named,
        _ => panic!("Fields in Struct must be named"),
    };

    let each_required_field = fields
        .clone()
        .into_iter()
        .filter_map(|field| {
            match field {
                Field {
                    ident,
                    ty: Type::Path(TypePath { path: Path { segments, .. }, qself: None }),
                    ..
                } => {
                    let name = match ident {
                        Some(name) => name,
                        None => panic!("Found unnamed field"),
                    };

                    let is_option = if let Some(&PathSegment { ref ident, .. }) = segments.first() {
                        ident == "Option"
                    } else {
                        false
                    };

                    if !is_option {
                        Some(
                            quote! { 
                                #name : match &self.#name {
                                    Some(#name) => { #name.clone() }
                                    None => {
                                        return Err(eyre!("setter was never called on your builder"));
                                    }
                                } 
                            }
                        )
                    } else {
                        None
                    }
                }
                _ => panic!("Expected only named fields"),
            }
        });

    let each_optional_field = fields
        .clone()
        .into_iter()
        .filter_map(|field| {
            match field {
                Field {
                    ident,
                    ty: Type::Path(TypePath { path: Path { segments, .. }, qself: None }),
                    ..
                } => {
                    let name = match ident {
                        Some(name) => name,
                        None => panic!("Found unnamed field"),
                    };

                    let is_option = if let Some(&PathSegment { ref ident, .. }) = segments.first() {
                        ident == "Option"
                    } else {
                        false
                    };

                    if is_option {
                        Some(
                            quote! { 
                            #name : match &self.#name {
                                Some(n) => { Some(n.clone()) }
                                None => None
                            } 
                        }
                        )
                    } else {
                        None
                    }
                }
                _ => panic!("Expected only named fields"),
            }
        });

    let expanded =
        quote! {
    use color_eyre::{ eyre::eyre, Result };

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
            Ok(Command { 
                #(#each_required_field ,)*
                #(#each_optional_field ,)*
            })
        }
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