use syn::{ Data, DataStruct, Error, DeriveInput, Fields, Result };

use crate::ExtractMut;

impl ExtractMut<Fields> for DeriveInput {
    fn extract_mut(&mut self) -> Result<&mut Fields> {
        match self.data {
            Data::Struct(DataStruct { struct_token: _, ref mut fields, semi_token: _ }) => {
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