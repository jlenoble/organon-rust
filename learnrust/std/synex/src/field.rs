use proc_macro2::Ident;
use syn::{
    Attribute,
    Error,
    Field,
    GenericArgument,
    Meta,
    Result,
    Type,
    TypePath,
    punctuated::Punctuated,
    token::Comma,
};

use crate::{ Extract, ExtractIter };

impl Extract<Ident> for Field {
    fn extract(&self) -> Result<&Ident> {
        match self {
            Field { attrs: _, ident: Some(ident), ty: _, vis: _, colon_token: _ } => { Ok(ident) }
            _ => { Err(Error::new_spanned(self, "failed to extract Ident from Field")) }
        }
    }
}

impl Extract<TypePath> for Field {
    fn extract(&self) -> Result<&TypePath> {
        match self.ty {
            Type::Path(ref typepath) => { Ok(typepath) }
            _ => {
                Err(Error::new_spanned(self, "expected Path as Type in Field, got another Type"))
            }
        }
    }
}

impl Extract<Punctuated<GenericArgument, Comma>> for Field {
    fn extract(&self) -> Result<&Punctuated<GenericArgument, Comma>> {
        let typepath: &TypePath = self.extract()?;
        typepath.extract()
    }
}

impl Extract<GenericArgument> for Field {
    fn extract(&self) -> Result<&GenericArgument> {
        let typepath: &TypePath = self.extract()?;
        typepath.extract()
    }
}

impl<'a> ExtractIter<'a, Meta> for &Field {
    type Iter = std::iter::Map<
        core::slice::Iter<'a, Attribute>,
        &'a dyn Fn(&'a Attribute) -> Result<Meta>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let Field { attrs, ident: _, ty: _, vis: _, colon_token: _ } = self;

        Ok(attrs.iter().map(&(|attribute: &'b Attribute| { attribute.parse_meta() })))
    }
}