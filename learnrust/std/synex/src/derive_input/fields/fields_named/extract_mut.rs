use syn::{ Field, FieldsNamed, Result, punctuated::Punctuated, token::Comma };

use crate::ExtractMut;

impl ExtractMut<Punctuated<Field, Comma>> for FieldsNamed {
    fn extract_mut(&mut self) -> Result<&mut Punctuated<Field, Comma>> {
        let FieldsNamed { brace_token: _, ref mut named } = self;
        Ok(named)
    }
}