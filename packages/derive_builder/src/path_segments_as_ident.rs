use color_eyre::{ Result, eyre::{ eyre, WrapErr } };
use syn::{ Ident, PathArguments, PathSegment, punctuated::Punctuated, token::Colon2 };

use crate::{ path_segments_as_segment::path_segments_as_segment, errors::ExtractionError };

pub fn path_segments_as_ident(segments: &Punctuated<PathSegment, Colon2>) -> Result<&Ident> {
    let segment = path_segments_as_segment(segments).wrap_err("failed to extract segment")?;

    let &PathSegment { ref ident, ref arguments } = segment;

    match arguments {
        PathArguments::None => Ok(ident),
        PathArguments::AngleBracketed(_) => { Ok(ident) }
        PathArguments::Parenthesized(_) => { Err(eyre!(ExtractionError::ParenthesizedArguments)) }
    }
}