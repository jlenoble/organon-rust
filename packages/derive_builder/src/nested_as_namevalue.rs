use color_eyre::{ Result, eyre::{ eyre, WrapErr } };
use proc_macro2::Ident;
use quote::format_ident;
use syn::{ Lit, Meta, NestedMeta };

use crate::{ errors::ExtractionError, path_as_ident::path_as_ident };

pub fn nested_as_namevalue(nested: &NestedMeta) -> Result<(&Ident, Ident)> {
    match nested {
        NestedMeta::Meta(Meta::NameValue(ref meta)) => {
            let name = path_as_ident(&meta.path).wrap_err("invalid name in NameValue")?;

            let value = if let Lit::Str(lit) = &meta.lit {
                format_ident!("{}", lit.value())
            } else {
                return Err(eyre!(ExtractionError::NotAString));
            };

            Ok((name, value))
        }
        NestedMeta::Meta(_) => { Err(eyre!(ExtractionError::NotNameValueMeta)) }
        NestedMeta::Lit(_) => { Err(eyre!(ExtractionError::NotNameValueMeta)) }
    }
}