use syn::{ Field, DeriveInput, FieldsUnnamed, Result };

use crate::{ Extract, ExtractNth };

impl ExtractNth<Field> for DeriveInput {
    fn extract_first(&self) -> Result<&Field> {
        FieldsUnnamed::extract_first(self.extract()?)
    }

    fn extract_last(&self) -> Result<&Field> {
        FieldsUnnamed::extract_last(self.extract()?)
    }

    fn extract_nth(&self, nth: usize) -> Result<&Field> {
        FieldsUnnamed::extract_nth(self.extract()?, nth)
    }
}