use quote::ToTokens;
use syn::{ Error, Result, punctuated::Punctuated };

use crate::Extract;

impl<T: ToTokens, P: ToTokens> Extract<T> for Punctuated<T, P> {
    fn extract(&self) -> Result<&T> {
        if self.len() == 1 {
            Ok(self.first().unwrap())
        } else {
            let type_name = std::any::type_name::<T>();
            let msg = format!(
                "expected only one {} in Punctuated<{},{}>",
                type_name,
                type_name,
                std::any::type_name::<P>()
            );

            Err(Error::new_spanned(self, msg))
        }
    }
}