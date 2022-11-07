use proc_macro2::Ident;
use syn::{ Error, GenericParam, Result, TypeParam };

use crate::Extract;

impl Extract<TypeParam> for GenericParam {
    fn extract(&self) -> Result<&TypeParam> {
        match self {
            GenericParam::Type(typeparam) => { Ok(typeparam) }
            GenericParam::Lifetime(_) => {
                Err(Error::new_spanned(self, "expected Type as GenericParam, got Lifetime"))
            }
            GenericParam::Const(_) => {
                Err(Error::new_spanned(self, "expected Type as GenericParam, got Const"))
            }
        }
    }
}

impl Extract<Ident> for GenericParam {
    fn extract(&self) -> Result<&Ident> {
        let typeparam: &TypeParam = self.extract()?;
        Ok(&typeparam.ident)
    }
}