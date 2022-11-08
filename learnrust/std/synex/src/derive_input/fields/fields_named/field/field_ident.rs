use proc_macro2::Ident;
use syn::{ Field, Result };

use crate::{ Extract, FieldIdent };

impl FieldIdent for Field {
    fn field_ident(&self) -> Result<&Ident> {
        self.extract()
    }
}