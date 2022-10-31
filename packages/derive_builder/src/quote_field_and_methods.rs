use proc_macro2::TokenStream;
use quote::{ quote, format_ident };
use syn::{ Field, Type, TypePath, Path, PathSegment, Meta, MetaList, NestedMeta, Lit };

pub fn quote_field_and_methods(field: &Field) -> (TokenStream, TokenStream) {
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