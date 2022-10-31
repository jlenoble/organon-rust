use color_eyre::{ Result, eyre::ensure };
use syn::{ PathSegment, punctuated::Punctuated, token::Colon2 };

use crate::errors::ExtractionError;

pub fn path_segments_as_segment(
    segments: &Punctuated<PathSegment, Colon2>
) -> Result<&PathSegment> {
    ensure!(segments.len() == 1, ExtractionError::NotExactlyOneSegment);

    Ok(segments.first().unwrap())
}