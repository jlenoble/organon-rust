use color_eyre::{ Result, eyre::ensure };
use syn::{ NestedMeta, punctuated::Punctuated, token::Comma };

use crate::errors::ExtractionError;

pub fn nested_meta_as_single_nested(nested: &Punctuated<NestedMeta, Comma>) -> Result<&NestedMeta> {
    ensure!(nested.len() == 1, ExtractionError::NotExactlyOneNested);

    Ok(nested.first().unwrap())
}