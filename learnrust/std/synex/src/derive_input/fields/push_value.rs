use syn::{ Field, Fields, FieldsNamed, Result };

use crate::{ PushValue, ExtractMut };

impl PushValue<Field> for Fields {
    fn push_value(&mut self, field: Field) -> Result<&mut Self> {
        let fieldsnamed: &mut FieldsNamed = self.extract_mut()?;
        fieldsnamed.push_value(field)?;
        Ok(self)
    }
}