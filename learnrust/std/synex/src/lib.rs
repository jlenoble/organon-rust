mod derive_input;
mod punctuated;

#[cfg(feature = "testsuite")]
mod testsuite;
#[cfg(feature = "testsuite")]
pub use testsuite::*;

use proc_macro2::Ident;
use quote::ToTokens;
use syn::Result;

pub trait Extract<T: ToTokens> where Self: ToTokens {
    fn extract(&self) -> Result<&T>;
}

pub trait ExtractNth<T: ToTokens> where Self: ToTokens {
    fn extract_first(&self) -> Result<&T>;
    fn extract_last(&self) -> Result<&T>;
    fn extract_nth(&self, nth: usize) -> Result<&T>;
}

pub trait ExtractWhere<'a, T: 'a + ToTokens> where Self: ToTokens {
    fn extract_where<'b: 'a>(
        &'b self,
        predicate: &'b dyn Fn(&'b T) -> Result<bool>
    ) -> Result<&'b T>
        where 'a: 'b;
}

pub trait FieldIdent where Self: ToTokens {
    fn field_ident(&self) -> Result<&Ident>;
}

pub trait NthFieldIdent where Self: ToTokens {
    fn first_field_ident(&self) -> Result<&Ident>;
    fn last_field_ident(&self) -> Result<&Ident>;
    fn nth_field_ident(&self, nth: usize) -> Result<&Ident>;
}

pub trait ExtractMut<T: ToTokens> where Self: ToTokens {
    fn extract_mut(&mut self) -> Result<&mut T>;
}

pub trait ExtractValue<T> where Self: ToTokens {
    fn extract_value(&self) -> Result<T>;
}

pub trait ExtractIter<'a, T: 'a + ToTokens>
    where Self: ToTokens, <<Self as ExtractIter<'a, T>>::Iter as Iterator>::Item: 'a
{
    type Iter: Iterator;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b;
}

pub trait PushValue<T: ToTokens> where Self: ToTokens {
    fn push_value(&mut self, t: T) -> Result<&mut Self>;
}