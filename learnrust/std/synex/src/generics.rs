use proc_macro2::Ident;
use syn::{
    Error,
    GenericParam,
    Generics,
    Result,
    TypeParam,
    punctuated::{ Iter, Punctuated },
    token::Comma,
};

use crate::{ PushValue, Extract, ExtractIter, ExtractMut };

impl Extract<Punctuated<GenericParam, Comma>> for Generics {
    fn extract(&self) -> Result<&Punctuated<GenericParam, Comma>> {
        let Generics { lt_token: _, ref params, gt_token: _, where_clause: _ } = self;
        Ok(params)
    }
}

impl ExtractMut<Punctuated<GenericParam, Comma>> for Generics {
    fn extract_mut(&mut self) -> Result<&mut Punctuated<GenericParam, Comma>> {
        let Generics { lt_token: _, ref mut params, gt_token: _, where_clause: _ } = self;
        Ok(params)
    }
}

impl<'a> ExtractIter<'a> for &Generics {
    type Iter = Iter<'a, GenericParam>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        Ok((Generics::extract(self)? as &Punctuated<GenericParam, Comma>).iter())
    }
}

impl Extract<GenericParam> for Generics {
    fn extract(&self) -> Result<&GenericParam> {
        let punct: &Punctuated<GenericParam, Comma> = self.extract()?;

        if punct.len() == 1 {
            Ok(punct.first().unwrap())
        } else {
            Err(Error::new_spanned(self, "expected only one GenericParam in Generics"))
        }
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

impl PushValue<GenericParam> for Generics {
    fn push_value(&mut self, param: GenericParam) -> Result<&mut Self> {
        let punct: &mut Punctuated<GenericParam, Comma> = self.extract_mut()?;
        punct.push_value(param);
        Ok(self)
    }
}

#[macro_export]
macro_rules! define_add_trait_bounds {
    ($($tt:tt)*) => {
        fn add_trait_bounds(mut generics: ::syn::Generics) -> ::syn::Generics {
            for param in &mut generics.params {
                if let ::syn::GenericParam::Type(ref mut type_param) = *param {
                    type_param.bounds.push(::syn::parse_quote!($($tt)*));
                }
            }
            generics
        }
    };
}