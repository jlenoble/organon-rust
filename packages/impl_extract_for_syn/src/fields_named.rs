use syn::{ Field, FieldsNamed, Result, punctuated::{ Iter, Punctuated }, token::Comma };

use crate::{ Extract, ExtractIter };

impl Extract<Punctuated<Field, Comma>> for FieldsNamed {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        let FieldsNamed { brace_token: _, ref named } = self;
        Ok(named)
    }
}

impl<'a> ExtractIter<'a> for &FieldsNamed {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        Ok(FieldsNamed::extract(self)?.iter())
    }
}