use proc_macro2::Ident;
use syn::{ Field, FieldsNamed, Result };

use crate::{ Extract, FieldByName, ExtractWhere };

impl FieldByName for FieldsNamed {
    fn field_by_name<'b>(&'b self, name: &'b str) -> Result<&'b Field> {
        let predicate = |field: &Field| {
            let ident: &Ident = field.extract()?;
            Ok(ident == name)
        };
        let field: &Field = self.extract_where(&predicate)?;
        Ok(field)
    }
}