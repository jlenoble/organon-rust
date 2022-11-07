use syn::{ DeriveInput, Field, Result, punctuated::{ Iter, Punctuated }, token::Comma };

use crate::{ Extract, ExtractIter };

impl<'a> ExtractIter<'a, Field> for &DeriveInput {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        Ok(punct.iter())
    }
}