use proc_macro2::Ident;
use syn::{ Field, Fields, FieldsNamed, Result };

use crate::{ Extract, FieldIdent };

impl FieldIdent for Fields {
    fn field_ident(&self) -> Result<&Ident> {
        let field: &Field = FieldsNamed::extract(self.extract()?)?;
        field.extract()
    }
}