use color_eyre::{ Result, eyre::WrapErr };
use proc_macro2::TokenStream;
use quote::{ quote, format_ident };
use syn::{ Field, Meta, MetaList, NestedMeta, Lit };

use crate::field_as_named_field_with_attributes_and_type::field_as_named_field_with_attributes_and_type;

pub fn quote_field_and_methods(field: &Field) -> Result<(TokenStream, TokenStream)> {
    let (field_name, type_name, attrs) = field_as_named_field_with_attributes_and_type(
        field
    ).wrap_err("field should be a named field with attributes and type")?;

    let (is_optional, is_vec) = (type_name == "Option", type_name == "Vec");

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
                                Some((lit, *field_name == lit2))
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
            #field_name : match &self.#field_name {
                Some(#field_name) => { Some(#field_name.clone()) }
                None => { None }
            } 
        }
    } else {
        quote! {
            #field_name : match &self.#field_name {
                Some(#field_name) => { #field_name.clone() }
                None => {
                    return Err(eyre!("setter was never called on your builder"));
                }
            }
        }
    };

    let vec_quot = if is_vec {
        quote! {
            fn #field_name(&mut self, #field_name: Vec<String>) -> &mut Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    } else {
        quote! {
            fn #field_name(&mut self, #field_name: String) -> &mut Self {
                self.#field_name = Some(#field_name);
                self
            }
        }
    };

    if let Some((inert_name, conflicting_method_names)) = inert_attr {
        if conflicting_method_names {
            Ok((
                quot,
                quote! {
                    fn #field_name(&mut self, #field_name: String) -> &mut Self {
                        if let Some(ref mut args) = self.args {
                            args.push(#field_name);
                        } else {
                            self.args = Some(vec![#field_name]);
                        }

                        self
                    }
                },
            ))
        } else {
            Ok((
                quot,
                quote! {
                    #vec_quot

                    fn #inert_name(&mut self, #inert_name: String) -> &mut Self {
                        if let Some(ref mut #field_name) = self.#field_name {
                            args.push(#inert_name);
                        } else {
                            self.args = Some(vec![#inert_name]);
                        }
    
                        self
                    }
                },
            ))
        }
    } else {
        Ok((quot, vec_quot))
    }
}