use syn::{ Error, Field, Fields, FieldsNamed, Result, punctuated::Punctuated, token::Comma };

use crate::ExtractMut;

impl ExtractMut<FieldsNamed> for Fields {
    fn extract_mut(&mut self) -> Result<&mut FieldsNamed> {
        match self {
            Fields::Named(fieldsnamed) => { Ok(fieldsnamed) }
            Fields::Unnamed(_) => {
                Err(Error::new_spanned(self, "expected FieldsNamed as Fields, got Unnamed"))
            }
            Fields::Unit => {
                Err(Error::new_spanned(self, "expected FieldsNamed as Fields, got Unit"))
            }
        }
    }
}

impl ExtractMut<Punctuated<Field, Comma>> for Fields {
    fn extract_mut(&mut self) -> Result<&mut Punctuated<Field, Comma>> {
        FieldsNamed::extract_mut(self.extract_mut()?)
    }
}