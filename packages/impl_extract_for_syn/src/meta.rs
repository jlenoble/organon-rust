use syn::{ Error, Meta, MetaList, MetaNameValue, Path, Result };

use crate::Extract;

impl Extract<MetaList> for Meta {
    fn extract(&self) -> Result<&MetaList> {
        match self {
            Meta::List(metalist) => { Ok(metalist) }
            Meta::Path(_) => Err(Error::new_spanned(self, "expected Meta as MetaList, got Path")),
            Meta::NameValue(_) =>
                Err(Error::new_spanned(self, "expected Meta as MetaList, got NameValue")),
        }
    }
}

impl Extract<Path> for Meta {
    fn extract(&self) -> Result<&Path> {
        match self {
            Meta::Path(path) => { Ok(path) }
            Meta::List(_) => Err(Error::new_spanned(self, "expected Meta as Path, got List")),
            Meta::NameValue(_) =>
                Err(Error::new_spanned(self, "expected Meta as Path, got NameValue")),
        }
    }
}

impl Extract<MetaNameValue> for Meta {
    fn extract(&self) -> Result<&MetaNameValue> {
        match self {
            Meta::NameValue(namevalue) => { Ok(namevalue) }
            Meta::List(_) => Err(Error::new_spanned(self, "expected Meta as NameValue, got List")),
            Meta::Path(_) => Err(Error::new_spanned(self, "expected Meta as NameValue, got Path")),
        }
    }
}