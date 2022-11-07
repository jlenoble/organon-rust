use syn::{ Error, Field, FieldsNamed, Result, punctuated::Punctuated, token::Comma };

use crate::Extract;

impl Extract<Punctuated<Field, Comma>> for FieldsNamed {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        let FieldsNamed { brace_token: _, ref named } = self;
        Ok(named)
    }
}

impl Extract<Field> for FieldsNamed {
    fn extract(&self) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;

        if punct.len() == 1 {
            Ok(punct.first().unwrap())
        } else {
            Err(Error::new_spanned(self, "expected only one Field in FieldsNamed"))
        }
    }
}