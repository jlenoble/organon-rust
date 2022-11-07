use proc_macro2::Ident;
use syn::{
    GenericParam,
    Generics,
    Result,
    TypeParam,
    punctuated::{ Iter, Punctuated },
    token::Comma,
};

use crate::{ Extract, ExtractIter };

impl<'a> ExtractIter<'a, GenericParam> for &Generics {
    type Iter = Iter<'a, GenericParam>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> {
        Ok((Generics::extract(self)? as &Punctuated<GenericParam, Comma>).iter())
    }
}

impl<'a> ExtractIter<'a, TypeParam> for &Generics {
    type Iter = std::iter::Map<
        <Self as ExtractIter<'a, GenericParam>>::Iter,
        &'a dyn Fn(&'a GenericParam) -> Result<&'a TypeParam>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let iter = <&Generics as ExtractIter<'b, GenericParam>>::extract_iter(self)?;
        Ok(iter.map(&(|generic_param: &'b GenericParam| { generic_param.extract() })))
    }
}

impl<'a> ExtractIter<'a, Ident> for &Generics {
    type Iter = std::iter::Map<
        <Self as ExtractIter<'a, GenericParam>>::Iter,
        &'a dyn Fn(&'a GenericParam) -> Result<&'a Ident>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let iter = <&Generics as ExtractIter<'b, GenericParam>>::extract_iter(self)?;
        Ok(iter.map(&(|generic_param: &'b GenericParam| { generic_param.extract() })))
    }
}