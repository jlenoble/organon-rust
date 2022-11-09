use proc_macro2::Ident;
use syn::{ DeriveInput, FieldsNamed, Result };

#[cfg(feature = "testsuite")]
use proc_macro2::TokenStream;
#[cfg(feature = "testsuite")]
use quote::quote;

use crate::{ Extract, NthFieldIdent };

impl NthFieldIdent for DeriveInput {
    fn first_field_ident(&self) -> Result<&Ident> {
        FieldsNamed::first_field_ident(self.extract()?)
    }

    fn last_field_ident(&self) -> Result<&Ident> {
        FieldsNamed::last_field_ident(self.extract()?)
    }

    fn nth_field_ident(&self, nth: usize) -> Result<&Ident> {
        FieldsNamed::nth_field_ident(self.extract()?, nth)
    }
}

#[cfg(feature = "testsuite")]
pub fn test_0th_first_last_field_names(derive_input: &DeriveInput) -> Result<TokenStream> {
    use syn::{ Data, DataStruct, Error, Field, Fields };

    let (first_field_name, last_field_name) = if
        let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }) =
            derive_input.data
    {
        let first_field = named.first();
        let last_field = named.last();

        if let Some(Field { ident: Some(ref field_name), .. }) = first_field {
            (field_name.to_string(), last_field.unwrap().ident.as_ref().unwrap().to_string())
        } else {
            return Err(
                Error::new_spanned(
                    named,
                    "expected to quote tests for a struct with at least one named field"
                )
            );
        }
    } else {
        return Err(
            Error::new_spanned(
                derive_input,
                "expected to quote tests for a struct with named fields"
            )
        );
    };

    let first_ident: &Ident = derive_input.first_field_ident()?;
    let first_ident = first_ident.to_string();

    let last_ident: &Ident = derive_input.last_field_ident()?;
    let last_ident = last_ident.to_string();

    let nth_0_ident: &Ident = derive_input.nth_field_ident(0)?;
    let nth_0_ident = nth_0_ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_extract_first_field_name() {
                assert_eq!(#first_ident, #first_field_name);
            }

            #[test]
            fn can_extract_last_field_name() {
                assert_eq!(#last_ident, #last_field_name);
            }

            #[test]
            fn can_extract_0th_field_name() {
                assert_eq!(#nth_0_ident, #first_field_name);
            }
        }
    )
}

#[cfg(feature = "testsuite")]
pub fn test_second_field_name(derive_input: &DeriveInput) -> Result<TokenStream> {
    use syn::{ Data, DataStruct, Error, Fields };

    let field_name = if
        let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }) =
            derive_input.data
    {
        if named.len() > 1 {
            let field = &named[1];

            if let Some(ref field_name) = field.ident {
                field_name.to_string()
            } else {
                return Err(Error::new_spanned(field, "expected field to be named"));
            }
        } else {
            return Err(
                Error::new_spanned(
                    named,
                    "expected to quote tests for a struct with at least two named fields"
                )
            );
        }
    } else {
        return Err(
            Error::new_spanned(
                derive_input,
                "expected to quote tests for a struct with named fields"
            )
        );
    };

    let ident: &Ident = derive_input.nth_field_ident(1)?;
    let ident = ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_extract_second_field_name() {
                assert_eq!(#ident, #field_name);
            }
        }
    )
}