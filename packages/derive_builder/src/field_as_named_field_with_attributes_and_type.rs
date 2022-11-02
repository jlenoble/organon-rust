use color_eyre::{ Result, eyre::{ bail } };
use proc_macro2::Ident;
use syn::{ Field, Type, Attribute };

use impl_extract_for_syn::Extract;

use crate::errors::ExtractionError;

pub fn field_as_named_field_with_attributes_and_type(
    field: &Field
) -> Result<(&Ident, &Ident, &Vec<Attribute>)> {
    if let Field { attrs, ident, ty: Type::Path(typepath), vis: _, colon_token: _ } = field {
        let field_name = if let Some(field_name) = ident {
            field_name
        } else {
            bail!(ExtractionError::UnnamedField);
        };

        let type_name = typepath.extract()?;

        Ok((field_name, type_name, attrs))
    } else {
        unreachable!()
    }
}