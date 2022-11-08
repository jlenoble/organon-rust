use syn::{ Field, FieldsNamed, Result, punctuated::Punctuated, token::Comma };

use crate::{ Extract, ExtractNth };

impl ExtractNth<Field> for FieldsNamed {
    fn extract_first(&self) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_first()
    }

    fn extract_last(&self) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_last()
    }

    fn extract_nth(&self, nth: usize) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_nth(nth)
    }
}