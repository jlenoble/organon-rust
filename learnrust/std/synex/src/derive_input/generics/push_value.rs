use syn::{ GenericParam, Generics, Result, punctuated::Punctuated, token::Comma };

use crate::{ PushValue, ExtractMut };

impl PushValue<GenericParam> for Generics {
    fn push_value(&mut self, param: GenericParam) -> Result<&mut Self> {
        let punct: &mut Punctuated<GenericParam, Comma> = self.extract_mut()?;
        punct.push_value(param);
        Ok(self)
    }
}