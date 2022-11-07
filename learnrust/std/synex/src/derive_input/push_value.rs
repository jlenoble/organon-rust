use syn::{ DeriveInput, Field, Fields, Result };

use crate::{ PushValue, ExtractMut };

impl PushValue<Field> for DeriveInput {
    fn push_value(&mut self, field: Field) -> Result<&mut Self> {
        let fields: &mut Fields = self.extract_mut()?;
        fields.push_value(field)?;
        Ok(self)
    }
}