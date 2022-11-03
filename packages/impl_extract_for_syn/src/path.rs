use proc_macro2::Ident;
use syn::{ Error, Path, PathSegment, Result, punctuated::Punctuated, token::Colon2 };

use crate::Extract;

impl Extract<Punctuated<PathSegment, Colon2>> for Path {
    fn extract(&self) -> Result<&Punctuated<PathSegment, Colon2>> {
        let Path { leading_colon: _, ref segments } = self;
        Ok(segments)
    }
}

impl Extract<PathSegment> for Path {
    fn extract(&self) -> Result<&PathSegment> {
        let segments: &Punctuated<PathSegment, Colon2> = self.extract()?;

        if segments.len() == 1 {
            Ok(segments.first().unwrap())
        } else {
            Err(Error::new_spanned(self, "expected only one PathSegment in Path"))
        }
    }
}

impl Extract<Ident> for Path {
    fn extract(&self) -> Result<&Ident> {
        let &PathSegment { ref ident, arguments: _ } = self.extract()?;
        Ok(ident)
    }
}