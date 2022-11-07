use proc_macro2::Ident;
use syn::{
    AngleBracketedGenericArguments,
    Error,
    GenericArgument,
    Path,
    PathSegment,
    Result,
    punctuated::Punctuated,
    token::{ Colon2, Comma },
    PathArguments,
};

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

impl Extract<Punctuated<GenericArgument, Comma>> for Path {
    fn extract(&self) -> Result<&Punctuated<GenericArgument, Comma>> {
        if
            let &PathSegment {
                ident: _,
                arguments: PathArguments::AngleBracketed(
                    AngleBracketedGenericArguments {
                        colon2_token: _,
                        lt_token: _,
                        ref args,
                        gt_token: _,
                    },
                ),
            } = self.extract()?
        {
            Ok(args)
        } else {
            Err(Error::new_spanned(self, "expected generic arguments"))
        }
    }
}

impl Extract<GenericArgument> for Path {
    fn extract(&self) -> Result<&GenericArgument> {
        let arguments: &Punctuated<GenericArgument, Comma> = self.extract()?;

        if arguments.len() == 1 {
            Ok(arguments.first().unwrap())
        } else {
            Err(Error::new_spanned(self, "expected only one GenericArgument for Path"))
        }
    }
}