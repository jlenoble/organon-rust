use syn::{ GenericParam, Generics, Result, punctuated::Punctuated, token::Comma };

use crate::ExtractMut;

impl ExtractMut<Punctuated<GenericParam, Comma>> for Generics {
    fn extract_mut(&mut self) -> Result<&mut Punctuated<GenericParam, Comma>> {
        let Generics { lt_token: _, ref mut params, gt_token: _, where_clause: _ } = self;
        Ok(params)
    }
}