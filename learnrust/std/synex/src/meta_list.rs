use proc_macro2::Ident;
use syn::{
    Error,
    MetaList,
    MetaNameValue,
    NestedMeta,
    Path,
    Result,
    punctuated::Punctuated,
    token::Comma,
};

use crate::Extract;

impl Extract<Path> for MetaList {
    fn extract(&self) -> Result<&Path> {
        Ok(&self.path)
    }
}

impl Extract<Ident> for MetaList {
    fn extract(&self) -> Result<&Ident> {
        self.path.extract()
    }
}

impl Extract<Punctuated<NestedMeta, Comma>> for MetaList {
    fn extract(&self) -> Result<&Punctuated<NestedMeta, Comma>> {
        Ok(&self.nested)
    }
}

impl Extract<NestedMeta> for MetaList {
    fn extract(&self) -> Result<&NestedMeta> {
        let segments: &Punctuated<NestedMeta, Comma> = self.extract()?;

        if segments.len() == 1 {
            Ok(segments.first().unwrap())
        } else {
            Err(Error::new_spanned(self, "expected only one NestedMeta segment in MetaList"))
        }
    }
}

impl Extract<MetaNameValue> for MetaList {
    fn extract(&self) -> Result<&MetaNameValue> {
        let nested_meta: &NestedMeta = self.extract()?;
        nested_meta.extract()
    }
}