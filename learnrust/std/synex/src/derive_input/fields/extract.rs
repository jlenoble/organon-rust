use syn::{ Error, Fields, FieldsNamed, FieldsUnnamed, Result };

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

impl Extract<FieldsUnnamed> for Fields {
    fn extract(&self) -> Result<&FieldsUnnamed> {
        match self {
            Fields::Unnamed(fieldsunnamed) => { Ok(fieldsunnamed) }
            Fields::Named(_) => {
                Err(Error::new_spanned(self, "expected FieldsUnnamed as Fields, got Named"))
            }
            Fields::Unit => {
                Err(Error::new_spanned(self, "expected FieldsUnnamed as Fields, got Unit"))
            }
        }
    }
}