use proc_macro2::Ident;
use syn::{ Field, Fields, Result };

use crate::{ Extract, FieldIdent };

impl FieldIdent for Fields {
    fn field_ident(&self) -> Result<&Ident> {
        let field: &Field = self.extract()?;
        field.extract()
    }
}