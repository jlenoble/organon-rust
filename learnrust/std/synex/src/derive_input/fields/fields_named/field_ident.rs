use proc_macro2::Ident;
use syn::{ Field, FieldsNamed, Result };

use crate::{ Extract, FieldIdent };

impl FieldIdent for FieldsNamed {
    fn field_ident(&self) -> Result<&Ident> {
        let field: &Field = self.extract()?;
        field.extract()
    }
}