use syn::{ Error, Meta, MetaNameValue, NestedMeta, Result };

use crate::Extract;

impl Extract<Meta> for NestedMeta {
    fn extract(&self) -> Result<&Meta> {
        match self {
            NestedMeta::Meta(meta) => { Ok(meta) }
            NestedMeta::Lit(_) => {
                Err(Error::new_spanned(self, "expected NestedMeta as Meta, got Lit"))
            }
        }
    }
}

impl Extract<MetaNameValue> for NestedMeta {
    fn extract(&self) -> Result<&MetaNameValue> {
        let meta: &Meta = self.extract()?;
        meta.extract()
    }
}