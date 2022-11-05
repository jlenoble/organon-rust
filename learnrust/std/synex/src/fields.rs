use syn::{
    Error,
    Field,
    Fields,
    FieldsNamed,
    Result,
    punctuated::{ Iter, Punctuated },
    token::Comma,
};

use crate::{ PushValue, Extract, ExtractIter, ExtractMut };

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

impl ExtractMut<FieldsNamed> for Fields {
    fn extract_mut(&mut self) -> Result<&mut FieldsNamed> {
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
        FieldsNamed::extract(self.extract()?)
    }
}

impl ExtractMut<Punctuated<Field, Comma>> for Fields {
    fn extract_mut(&mut self) -> Result<&mut Punctuated<Field, Comma>> {
        FieldsNamed::extract_mut(self.extract_mut()?)
    }
}

impl Extract<Field> for Fields {
    fn extract(&self) -> Result<&Field> {
        FieldsNamed::extract(self.extract()?)
    }
}

impl<'a> ExtractIter<'a, Field> for &Fields {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        Ok(punct.iter())
    }
}

impl PushValue<Field> for Fields {
    fn push_value(&mut self, field: Field) -> Result<&mut Self> {
        let fieldsnamed: &mut FieldsNamed = self.extract_mut()?;
        fieldsnamed.push_value(field)?;
        Ok(self)
    }
}