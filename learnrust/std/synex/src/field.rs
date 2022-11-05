use proc_macro2::Ident;
use syn::{ Attribute, Error, Field, Meta, Result, Type, TypePath };

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

impl<'a> ExtractIter<'a, Field> for &Field {
    type Iter = std::iter::MapWhile<
        core::slice::Iter<'a, Attribute>,
        &'a dyn Fn(&'a Attribute) -> Option<Meta>
    >;

    fn extract_iter<'b: 'a>(&'b self) -> Result<Self::Iter> where 'a: 'b {
        let Field { attrs, ident: _, ty: _, vis: _, colon_token: _ } = self;

        Ok(attrs.iter().map_while(&(|attribute: &'b Attribute| { attribute.parse_meta().ok() })))
    }
}