use proc_macro2::Ident;
use syn::{
    Error,
    Path,
    PathArguments,
    PathSegment,
    Result,
    TypePath,
    punctuated::Punctuated,
    token::Colon2,
};

use crate::Extract;

impl Extract<Path> for TypePath {
    fn extract(&self) -> Result<&Path> {
        let TypePath { path, qself } = self;

        if qself.is_some() {
            return Err(
                Error::new_spanned(
                    self,
                    "path should not be qualified, as in <Self as HasItem>::Item"
                )
            );
        }

        Ok(path)
    }
}

impl Extract<Punctuated<PathSegment, Colon2>> for TypePath {
    fn extract(&self) -> Result<&Punctuated<PathSegment, Colon2>> {
        let Path { leading_colon, ref segments } = self.extract()?;

        if leading_colon.is_some() {
            return Err(Error::new_spanned(self, "found leading double colon in path"));
        }

        Ok(segments)
    }
}

impl Extract<PathSegment> for TypePath {
    fn extract(&self) -> Result<&PathSegment> {
        let segments: &Punctuated<PathSegment, Colon2> = self.extract()?;

        if segments.len() == 1 {
            Ok(segments.first().unwrap())
        } else {
            Err(Error::new_spanned(self, "expected only one segment"))
        }
    }
}

impl Extract<Ident> for TypePath {
    fn extract(&self) -> Result<&Ident> {
        let &PathSegment { ref ident, ref arguments } = self.extract()?;

        match arguments {
            PathArguments::Parenthesized(_) => {
                Err(
                    Error::new_spanned(
                        self,
                        "found parenthesized arguments in segment, as the (A, B) -> C in Fn(A, B) -> C."
                    )
                )
            }
            _ => Ok(ident),
        }
    }
}