use syn::{ Field, Fields, Result, punctuated::Punctuated, token::Comma };

use crate::{ Extract, ExtractWhere };

impl<'a> ExtractWhere<'a, Field> for Fields {
    fn extract_where<'b: 'a>(
        &'b self,
        predicate: &'b dyn Fn(&'b Field) -> Result<bool>
    ) -> Result<&'b Field>
        where 'a: 'b
    {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_where(predicate)
    }
}