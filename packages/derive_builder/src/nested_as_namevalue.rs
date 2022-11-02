use color_eyre::{ Result, eyre::{ eyre } };
use proc_macro2::Ident;
use quote::format_ident;
use syn::{ Lit, Meta, NestedMeta };

use impl_extract_for_syn::Extract;

use crate::errors::ExtractionError;

pub fn nested_as_namevalue(nested: &NestedMeta) -> Result<(&Ident, Ident)> {
    match nested {
        NestedMeta::Meta(Meta::NameValue(ref meta)) => {
            let name = meta.path.extract()?;

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