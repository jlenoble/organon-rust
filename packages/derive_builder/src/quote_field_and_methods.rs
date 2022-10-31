use color_eyre::{ Result, eyre::WrapErr };
use proc_macro2::TokenStream;
use quote::{ quote, format_ident };
use syn::{ Field, Meta, MetaList, NestedMeta, Lit };

use crate::{
    field_as_named_field_with_attributes_and_type::field_as_named_field_with_attributes_and_type,
    quote_methods::{
        quote_all_at_once_setter_method,
        quote_one_at_a_time_setter_method,
        quote_both_setter_methods,
    },
    quote_field::quote_field,
};

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

    let quote_field = quote_field(field_name, is_optional);

    let quote_methods = if let Some((ref inert_name, conflicting_method_names)) = inert_attr {
        if conflicting_method_names {
            quote_one_at_a_time_setter_method(field_name, inert_name)
        } else {
            quote_both_setter_methods(field_name, inert_name)
        }
    } else {
        quote_all_at_once_setter_method(field_name, if is_vec {
            quote! { Vec<String> }
        } else {
            quote! { String }
        })
    };

    Ok((quote_field, quote_methods))
}