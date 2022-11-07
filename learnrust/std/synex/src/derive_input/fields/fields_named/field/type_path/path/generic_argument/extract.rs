use proc_macro2::Ident;
use syn::{ Error, GenericArgument, Result, Type, TypePath };

use crate::Extract;

impl Extract<TypePath> for GenericArgument {
    fn extract(&self) -> Result<&TypePath> {
        match self {
            GenericArgument::Type(Type::Path(typepath)) => { Ok(typepath) }
            GenericArgument::Type(_) => {
                Err(
                    Error::new_spanned(
                        self,
                        "expected TypePath as GenericArgument, got another Type"
                    )
                )
            }
            GenericArgument::Lifetime(_) => {
                Err(Error::new_spanned(self, "expected Type as GenericArgument, got Lifetime"))
            }
            GenericArgument::Const(_) => {
                Err(Error::new_spanned(self, "expected Type as GenericArgument, got Const"))
            }
            GenericArgument::Binding(_) => {
                Err(Error::new_spanned(self, "expected Type as GenericArgument, got Binding"))
            }
            GenericArgument::Constraint(_) => {
                Err(Error::new_spanned(self, "expected Type as GenericArgument, got Constraint"))
            }
        }
    }
}

impl Extract<Ident> for GenericArgument {
    fn extract(&self) -> Result<&Ident> {
        let typepath: &TypePath = self.extract()?;
        typepath.extract()
    }
}