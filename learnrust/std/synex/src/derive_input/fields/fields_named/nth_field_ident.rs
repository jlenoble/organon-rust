use proc_macro2::Ident;
use syn::{ Field, FieldsNamed, Result };

use crate::{ Extract, ExtractNth, NthFieldIdent };

impl NthFieldIdent for FieldsNamed {
    fn first_field_ident(&self) -> Result<&Ident> {
        let field: &Field = self.extract_first()?;
        field.extract()
    }

    fn last_field_ident(&self) -> Result<&Ident> {
        let field: &Field = self.extract_last()?;
        field.extract()
    }

    fn nth_field_ident(&self, nth: usize) -> Result<&Ident> {
        let field: &Field = self.extract_nth(nth)?;
        field.extract()
    }
}