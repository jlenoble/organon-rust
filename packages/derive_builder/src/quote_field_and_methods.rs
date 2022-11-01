use color_eyre::{ Result, eyre::WrapErr };
use proc_macro2::TokenStream;
use syn::Field;

use crate::{
    attribute_as_path_with_name_value::attribute_as_path_with_name_value,
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
        let (inert, name, value) = attribute_as_path_with_name_value(attrs.first().unwrap())?;

        if inert == "builder" && name == "each" {
            Some(value)
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