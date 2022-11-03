use syn::{
    Error,
    Field,
    Fields,
    FieldsNamed,
    Result,
    punctuated::{ Iter, Punctuated },
    token::Comma,
};

use crate::{ Extract, ExtractIter };

impl Extract<FieldsNamed> for Fields {
    fn extract(&self) -> Result<&FieldsNamed> {
        match self {
            Fields::Named(fieldsnamed) => { Ok(fieldsnamed) }
            Fields::Unnamed(_) => {
                Err(Error::new_spanned(self, "expected FieldsNamed as Fields, got Unnamed"))
            }
            Fields::Unit => {
                Err(Error::new_spanned(self, "expected FieldsNamed as Fields, got Unit"))
            }
        }
    }
}

impl Extract<Punctuated<Field, Comma>> for Fields {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        FieldsNamed::extract(Fields::extract(self)?)
    }
}

impl<'a> ExtractIter<'a> for &Fields {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        Ok(punct.iter())
    }
}