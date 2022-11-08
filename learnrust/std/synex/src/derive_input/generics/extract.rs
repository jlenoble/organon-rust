use proc_macro2::Ident;
use syn::{ GenericParam, Generics, Result, TypeParam, punctuated::Punctuated, token::Comma };

use crate::Extract;

impl Extract<Punctuated<GenericParam, Comma>> for Generics {
    fn extract(&self) -> Result<&Punctuated<GenericParam, Comma>> {
        let Generics { lt_token: _, ref params, gt_token: _, where_clause: _ } = self;
        Ok(params)
    }
}

impl Extract<GenericParam> for Generics {
    fn extract(&self) -> Result<&GenericParam> {
        let punct: &Punctuated<GenericParam, Comma> = self.extract()?;
        punct.extract()
    }
}

impl Extract<TypeParam> for Generics {
    fn extract(&self) -> Result<&TypeParam> {
        let generic_param: &GenericParam = self.extract()?;
        generic_param.extract()
    }
}

impl Extract<Ident> for Generics {
    fn extract(&self) -> Result<&Ident> {
        let generic_param: &GenericParam = self.extract()?;
        generic_param.extract()
    }
}