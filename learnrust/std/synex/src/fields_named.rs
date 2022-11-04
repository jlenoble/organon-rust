use syn::{ Field, FieldsNamed, Result, punctuated::{ Iter, Punctuated }, token::Comma };

use crate::{ PushValue, Extract, ExtractIter, ExtractMut };

impl Extract<Punctuated<Field, Comma>> for FieldsNamed {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        let FieldsNamed { brace_token: _, ref named } = self;
        Ok(named)
    }
}

impl ExtractMut<Punctuated<Field, Comma>> for FieldsNamed {
    fn extract_mut(&mut self) -> Result<&mut Punctuated<Field, Comma>> {
        let FieldsNamed { brace_token: _, ref mut named } = self;
        Ok(named)
    }
}

impl<'a> ExtractIter<'a> for &FieldsNamed {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        Ok(FieldsNamed::extract(self)?.iter())
    }
}

impl PushValue<Field> for FieldsNamed {
    fn push_value(&mut self, field: Field) -> Result<&mut Self> {
        let punct: &mut Punctuated<Field, Comma> = self.extract_mut()?;
        punct.push_value(field);
        Ok(self)
    }
}