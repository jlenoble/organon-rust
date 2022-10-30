use proc_macro2::TokenStream;
use quote::{ quote, format_ident };
use syn::{
    parse_macro_input,
    DeriveInput,
    Data,
    Fields,
    Field,
    Type,
    TypePath,
    Path,
    PathSegment,
    Meta,
    MetaList,
    NestedMeta,
    Lit,
};

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

fn quote_field_and_methods(field: &Field) -> (TokenStream, TokenStream) {
    let (name, segments, attrs) = if
        let Field {
            ref attrs,
            ref ident,
            ty: Type::Path(TypePath { path: Path { ref segments, .. }, qself: None }),
            ..
        } = field
    {
        (ident, segments, attrs)
    } else {
        panic!("Not a field");
    };

    let name = match name {
        Some(name) => name,
        None => panic!("Found unnamed field"),
    };

    let (is_optional, is_vec) = if let Some(&PathSegment { ref ident, .. }) = segments.first() {
        (ident == "Option", ident == "Vec")
    } else {
        (false, false)
    };

    let inert_attr = if !attrs.is_empty() {
        if let Some(meta) = attrs.first() {
            if let Ok(Meta::List(MetaList { ref path, ref nested, .. })) = meta.parse_meta() {
                if path.is_ident("builder") {
                    if
                        let Some(NestedMeta::Meta(Meta::NameValue(meta))) =
                            Vec::from_iter(nested).first()
                    {
                        if meta.path.is_ident("each") {
                            if let Lit::Str(lit) = &meta.lit {
                                let lit = format_ident!("{}", lit.value());
                                let lit2 = lit.clone();
                                Some((lit, *name == lit2))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    let quot = if is_optional {
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
    };

    let vec_quot = if is_vec {
        quote! {
            fn #name(&mut self, #name: Vec<String>) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    } else {
        quote! {
            fn #name(&mut self, #name: String) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    };

    if let Some((inert_name, conflicting_method_names)) = inert_attr {
        if conflicting_method_names {
            (
                quot,
                quote! {
                    fn #name(&mut self, #name: String) -> &mut Self {
                        if let Some(ref mut args) = self.args {
                            args.push(#name);
                        } else {
                            self.args = Some(vec![#name]);
                        }

                        self
                    }
                },
            )
        } else {
            (
                quot,
                quote! {
                    #vec_quot

                    fn #inert_name(&mut self, #inert_name: String) -> &mut Self {
                        if let Some(ref mut #name) = self.#name {
                            args.push(#inert_name);
                        } else {
                            self.args = Some(vec![#inert_name]);
                        }
    
                        self
                    }
                },
            )
        }
    } else {
        (quot, vec_quot)
    }
}

fn quote_fields_and_methods(data: &Data) -> (TokenStream, TokenStream) {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let (each_field, each_method): (
                        Vec<TokenStream>,
                        Vec<TokenStream>,
                    ) = fields.named
                        .iter()
                        .map(|field| { quote_field_and_methods(field) })
                        .unzip();

                    (
                        quote! {
                        #(#each_field ,)*
                    },
                        quote! { #(#each_method)* },
                    )
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