use proc_macro2::Ident;
use syn::{ Fields, FieldsNamed, Result };

use crate::{ Extract, NthFieldIdent };

impl NthFieldIdent for Fields {
    fn first_field_ident(&self) -> Result<&Ident> {
        FieldsNamed::first_field_ident(self.extract()?)
    }

    fn last_field_ident(&self) -> Result<&Ident> {
        FieldsNamed::last_field_ident(self.extract()?)
    }

    fn nth_field_ident(&self, nth: usize) -> Result<&Ident> {
        FieldsNamed::nth_field_ident(self.extract()?, nth)
    }
}