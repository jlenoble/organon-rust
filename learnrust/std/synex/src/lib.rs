use quote::ToTokens;
use syn::Result;

pub trait Extract<T: ToTokens> where Self: ToTokens {
    fn extract(&self) -> Result<&T>;
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

mod derive_input;

pub mod type_path;
pub mod path;

pub mod meta;
pub mod meta_list;
pub mod nested_meta;

pub mod lit;

pub mod generic_argument;