use color_eyre::{ Result, eyre::{ ensure, WrapErr } };
use syn::{ Ident, Path };

use crate::{ path_segments_as_ident::path_segments_as_ident, errors::ExtractionError };

pub fn path_as_ident(path: &Path) -> Result<&Ident> {
    let Path { segments, leading_colon } = path;

    ensure!(leading_colon.is_none(), ExtractionError::LeadingDoubleColon);

    let ident = path_segments_as_ident(segments).wrap_err("failed to extract ident in Segments")?;

    Ok(ident)
}