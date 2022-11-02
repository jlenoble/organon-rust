use quote::ToTokens;
use syn::Result;

pub trait Extract<T: ToTokens> where Self: ToTokens {
    fn extract(&self) -> Result<&T>;
}

pub trait ExtractIter<'a>
    where Self: ToTokens, <<Self as ExtractIter<'a>>::Iter as Iterator>::Item: 'a + ToTokens
{
    type Iter: Iterator;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter>;
}

pub mod derive_input;
pub mod fields;
pub mod fields_named;

pub mod type_path;
pub mod path;