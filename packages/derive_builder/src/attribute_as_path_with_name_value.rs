use color_eyre::{ Result, eyre::{ eyre, WrapErr } };
use proc_macro2::Ident;
use syn::{ Attribute, Meta, MetaList };

use impl_extract_for_syn::Extract;

use crate::{
    nested_meta_as_single_nested::nested_meta_as_single_nested,
    nested_as_namevalue::nested_as_namevalue,
    errors::ExtractionError,
};

pub fn attribute_as_path_with_name_value(attribute: &Attribute) -> Result<(Ident, Ident, Ident)> {
    if
        let Meta::List(MetaList { path, nested, paren_token: _ }) = attribute
            .parse_meta()
            .wrap_err("failed to parse attribute as MetaList")?
    {
        let inert_attribute: &Ident = path.extract()?;

        let nested_meta = nested_meta_as_single_nested(&nested).wrap_err(
            "too many nested metas or none"
        )?;

        let (name, value) = nested_as_namevalue(&nested_meta).wrap_err(
            "failed to extract name = value"
        )?;

        Ok((inert_attribute.clone(), name.clone(), value))
    } else {
        Err(eyre!(ExtractionError::NotAMetaList))
    }
}