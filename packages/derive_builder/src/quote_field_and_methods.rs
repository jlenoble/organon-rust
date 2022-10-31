use color_eyre::{ Result, eyre::WrapErr };
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::{ Field, Meta, MetaList, NestedMeta, Lit };

use crate::{
    field_as_named_field_with_attributes_and_type::field_as_named_field_with_attributes_and_type,
    quote_methods::quote_methods,
    quote_field::quote_field,
};

pub fn quote_field_and_methods(field: &Field) -> Result<(TokenStream, TokenStream)> {
    let (field_name, type_name, attrs) = field_as_named_field_with_attributes_and_type(
        field
    ).wrap_err("field should be a named field with attributes and type")?;

    let (is_optional, is_vec) = (type_name == "Option", type_name == "Vec");

    let method_name = if !attrs.is_empty() {
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
                                Some(lit)
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

    let method_name = if let Some(ref method_name) = method_name {
        Some(method_name)
    } else {
        None
    };

    let quote_field = quote_field(field_name, is_optional);
    let quote_methods = quote_methods(field_name, method_name, is_vec);

    Ok((quote_field, quote_methods))
}