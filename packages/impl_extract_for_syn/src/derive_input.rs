use syn::{
    Data,
    DataStruct,
    Error,
    DeriveInput,
    Field,
    Fields,
    Result,
    FieldsNamed,
    punctuated::{ Iter, Punctuated },
    token::Comma,
};

use crate::{ Extract, ExtractIter };

impl Extract<Fields> for DeriveInput {
    fn extract(&self) -> Result<&Fields> {
        match self.data {
            Data::Struct(DataStruct { struct_token: _, ref fields, semi_token: _ }) => {
                Ok(fields)
            }
            Data::Enum(_) => {
                Err(Error::new_spanned(self, "expected Struct as Data in DeriveInput, got Enum"))
            }
            Data::Union(_) => {
                Err(Error::new_spanned(self, "expected Struct as Data in DeriveInput, got Union"))
            }
        }
    }
}

impl Extract<FieldsNamed> for DeriveInput {
    fn extract(&self) -> Result<&FieldsNamed> {
        Fields::extract(DeriveInput::extract(self)?)
    }
}

impl Extract<Punctuated<Field, Comma>> for DeriveInput {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        FieldsNamed::extract(DeriveInput::extract(self)?)
    }
}

impl<'a> ExtractIter<'a> for &DeriveInput {
    type Iter = Iter<'a, Field>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        Ok(punct.iter())
    }
}