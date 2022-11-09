use quote::ToTokens;
use syn::{ Error, Result, punctuated::Punctuated };

use crate::ExtractWhere;

impl<T: ToTokens, P: ToTokens> ExtractWhere<T> for Punctuated<T, P> {
    fn extract_where<'b>(&'b self, predicate: &dyn Fn(&'b T) -> Result<bool>) -> Result<&'b T> {
        if self.is_empty() {
            let type_name = std::any::type_name::<T>();

            return Err(
                Error::new_spanned(
                    self,
                    format!(
                        "failed to conditionally extract {} in Punctuated<{},{}>: it is empty",
                        type_name,
                        type_name,
                        std::any::type_name::<P>()
                    )
                )
            );
        }

        let mut found: Option<&'b T> = None;

        for t in self.iter() {
            if predicate(t)? {
                if found.is_none() {
                    found = Some(t);
                } else {
                    let type_name = std::any::type_name::<T>();

                    return Err(
                        Error::new_spanned(
                            self,
                            format!(
                                "failed to conditionally extract {} in Punctuated<{},{}>: more than one occurrence",
                                type_name,
                                type_name,
                                std::any::type_name::<P>()
                            )
                        )
                    );
                }
            }
        }

        if let Some(found) = found {
            Ok(found)
        } else {
            let type_name = std::any::type_name::<T>();

            Err(
                Error::new_spanned(
                    self,
                    format!(
                        "failed to conditionally extract {} in Punctuated<{},{}>: predicate matches nothing",
                        type_name,
                        type_name,
                        std::any::type_name::<P>()
                    )
                )
            )
        }
    }
}