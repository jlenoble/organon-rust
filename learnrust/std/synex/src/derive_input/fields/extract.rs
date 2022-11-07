use syn::{ Error, Field, Fields, FieldsNamed, Result, punctuated::Punctuated, token::Comma };

use crate::Extract;

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
        FieldsNamed::extract(self.extract()?)
    }
}

impl Extract<Field> for Fields {
    fn extract(&self) -> Result<&Field> {
        FieldsNamed::extract(self.extract()?)
    }
}