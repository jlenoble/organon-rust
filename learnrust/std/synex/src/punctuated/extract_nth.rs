use quote::ToTokens;
use syn::{ Error, Result, punctuated::Punctuated };

use crate::ExtractNth;

impl<T: ToTokens, P: ToTokens> ExtractNth<T> for Punctuated<T, P> {
    fn extract_first(&self) -> Result<&T> {
        if let Some(item) = self.first() {
            Ok(item)
        } else {
            let type_name = std::any::type_name::<T>();
            let msg = format!(
                "expected at least one {} in Punctuated<{},{}>",
                type_name,
                type_name,
                std::any::type_name::<P>()
            );

            Err(Error::new_spanned(self, msg))
        }
    }

    fn extract_last(&self) -> Result<&T> {
        if let Some(item) = self.last() {
            Ok(item)
        } else {
            let type_name = std::any::type_name::<T>();
            let msg = format!(
                "expected at least one {} in Punctuated<{},{}>",
                type_name,
                type_name,
                std::any::type_name::<P>()
            );

            Err(Error::new_spanned(self, msg))
        }
    }

    fn extract_nth(&self, nth: usize) -> Result<&T> {
        if self.len() > nth {
            Ok(&self[nth])
        } else {
            Err(
                Error::new_spanned(
                    self,
                    format!(
                        "out of bound in Punctuated<{},{}>: index {} is too large",
                        std::any::type_name::<T>(),
                        std::any::type_name::<P>(),
                        nth
                    )
                )
            )
        }
    }
}