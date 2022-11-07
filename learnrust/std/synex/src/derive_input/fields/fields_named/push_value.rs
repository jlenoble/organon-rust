use syn::{ Field, FieldsNamed, Result, punctuated::Punctuated, token::Comma };

use crate::{ PushValue, ExtractMut };

impl PushValue<Field> for FieldsNamed {
    fn push_value(&mut self, field: Field) -> Result<&mut Self> {
        let punct: &mut Punctuated<Field, Comma> = self.extract_mut()?;
        punct.push_value(field);
        Ok(self)
    }
}