use syn::{ Field, FieldsNamed, Result, punctuated::{ Iter, Punctuated }, token::Comma };

use crate::{ Extract, ExtractIter };

impl<'a> ExtractIter<'a, Field> for &FieldsNamed {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        Ok((FieldsNamed::extract(self)? as &Punctuated<Field, Comma>).iter())
    }
}