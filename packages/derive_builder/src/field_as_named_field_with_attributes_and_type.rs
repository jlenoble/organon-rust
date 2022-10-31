use color_eyre::{ Result, eyre::{ bail, WrapErr } };
use proc_macro2::Ident;
use syn::{ Field, Type, Attribute };

use crate::{ typepath_as_ident::typepath_as_ident, errors::ExtractionError };

pub fn field_as_named_field_with_attributes_and_type(
    field: &Field
) -> Result<(&Ident, &Ident, &Vec<Attribute>)> {
    if let Field { attrs, ident, ty: Type::Path(typepath), vis: _, colon_token: _ } = field {
        let field_name = if let Some(field_name) = ident {
            field_name
        } else {
            bail!(ExtractionError::UnnamedField);
        };

        let type_name = typepath_as_ident(typepath).wrap_err("failed to extract type from field")?;

        Ok((field_name, type_name, attrs))
    } else {
        unreachable!()
    }
}