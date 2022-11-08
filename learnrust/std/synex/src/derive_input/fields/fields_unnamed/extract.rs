use syn::{ Field, FieldsUnnamed, Result, punctuated::Punctuated, token::Comma };

use crate::Extract;

impl Extract<Punctuated<Field, Comma>> for FieldsUnnamed {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        let FieldsUnnamed { paren_token: _, ref unnamed } = self;
        Ok(unnamed)
    }
}

impl Extract<Field> for FieldsUnnamed {
    fn extract(&self) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract()
    }
}