use syn::{ Field, FieldsUnnamed, Result, punctuated::Punctuated, token::Comma };

use crate::{ Extract, ExtractWhere };

impl ExtractWhere<Field> for FieldsUnnamed {
    fn extract_where<'b>(
        &'b self,
        predicate: &dyn Fn(&'b Field) -> Result<bool>
    ) -> Result<&'b Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_where(predicate)
    }
}