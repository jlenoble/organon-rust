use proc_macro2::Ident;
use syn::{
    Attribute,
    Field,
    GenericArgument,
    Meta,
    Result,
    TypePath,
    punctuated::{ Iter, Punctuated },
    token::Comma,
};

use crate::{ Extract, ExtractIter };

impl<'a> ExtractIter<'a, GenericArgument> for &Field {
    type Iter = Iter<'a, GenericArgument>;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let punct: &Punctuated<GenericArgument, Comma> = self.extract()?;
        Ok(punct.iter())
    }
}

impl<'a> ExtractIter<'a, TypePath> for &Field {
    type Iter = std::iter::Map<
        Iter<'a, GenericArgument>,
        &'a dyn Fn(&'a GenericArgument) -> Result<&'a TypePath>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let punct: &Punctuated<GenericArgument, Comma> = self.extract()?;
        Ok(punct.iter().map(&(|generic_argument| { generic_argument.extract() })))
    }
}

impl<'a> ExtractIter<'a, Ident> for &Field {
    type Iter = std::iter::Map<
        Iter<'a, GenericArgument>,
        &'a dyn Fn(&'a GenericArgument) -> Result<&'a Ident>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let punct: &Punctuated<GenericArgument, Comma> = self.extract()?;
        Ok(punct.iter().map(&(|generic_argument| { generic_argument.extract() })))
    }
}

impl<'a> ExtractIter<'a, Meta> for &Field {
    type Iter = std::iter::Map<
        core::slice::Iter<'a, Attribute>,
        &'a dyn Fn(&'a Attribute) -> Result<Meta>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let Field { attrs, ident: _, ty: _, vis: _, colon_token: _ } = self;

        Ok(attrs.iter().map(&(|attribute: &'b Attribute| { attribute.parse_meta() })))
    }
}