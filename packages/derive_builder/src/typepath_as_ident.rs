use color_eyre::{ Result, eyre::{ ensure, WrapErr } };
use syn::{ Ident, TypePath };

use crate::{ path_as_ident::path_as_ident, errors::ExtractionError };

pub fn typepath_as_ident(typepath: &TypePath) -> Result<&Ident> {
    let TypePath { ref path, qself } = typepath;

    ensure!(qself.is_none(), ExtractionError::QualifiedPath);

    let ident = path_as_ident(path).wrap_err("failed to extract ident in Path")?;

    Ok(ident)
}