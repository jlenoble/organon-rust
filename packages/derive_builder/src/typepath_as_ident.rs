use color_eyre::{ Result, eyre::{ ensure, WrapErr } };
use syn::{ Ident, Path, TypePath };

use crate::{ path_segments_as_ident::path_segments_as_ident, errors::ExtractionError };

pub fn typepath_as_ident(typepath: &TypePath) -> Result<&Ident> {
    let TypePath { path: Path { segments, leading_colon }, qself } = typepath;

    ensure!(leading_colon.is_none(), ExtractionError::LeadingDoubleColon);
    ensure!(qself.is_none(), ExtractionError::QualifiedPath);

    let ident = path_segments_as_ident(segments).wrap_err("failed to extract ident")?;

    Ok(ident)
}