use proc_macro2::Ident;
use syn::{
    GenericArgument,
    Path,
    PathSegment,
    Result,
    TypePath,
    punctuated::Punctuated,
    token::{ Colon2, Comma },
};

use crate::Extract;

impl Extract<Path> for TypePath {
    fn extract(&self) -> Result<&Path> {
        let TypePath { path, qself: _ } = self;
        Ok(path)
    }
}

impl Extract<Punctuated<PathSegment, Colon2>> for TypePath {
    fn extract(&self) -> Result<&Punctuated<PathSegment, Colon2>> {
        Path::extract(self.extract()?)
    }
}

impl Extract<PathSegment> for TypePath {
    fn extract(&self) -> Result<&PathSegment> {
        Path::extract(self.extract()?)
    }
}

impl Extract<Ident> for TypePath {
    fn extract(&self) -> Result<&Ident> {
        Path::extract(self.extract()?)
    }
}

impl Extract<Punctuated<GenericArgument, Comma>> for TypePath {
    fn extract(&self) -> Result<&Punctuated<GenericArgument, Comma>> {
        let path: &Path = self.extract()?;
        path.extract()
    }
}

impl Extract<GenericArgument> for TypePath {
    fn extract(&self) -> Result<&GenericArgument> {
        let path: &Path = self.extract()?;
        path.extract()
    }
}