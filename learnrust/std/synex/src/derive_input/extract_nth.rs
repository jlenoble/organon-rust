use syn::{ Field, DeriveInput, Result, punctuated::Punctuated, token::Comma };

#[cfg(feature = "testsuite")]
use proc_macro2::TokenStream;
#[cfg(feature = "testsuite")]
use quote::quote;

use crate::{ Extract, ExtractNth };

impl ExtractNth<Field> for DeriveInput {
    fn extract_first(&self) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_first()
    }

    fn extract_last(&self) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_last()
    }

    fn extract_nth(&self, nth: usize) -> Result<&Field> {
        let punct: &Punctuated<Field, Comma> = self.extract()?;
        punct.extract_nth(nth)
    }
}

#[cfg(feature = "testsuite")]
pub fn quote_one_named_field_ident(derive_input: &DeriveInput) -> Result<TokenStream> {
    use proc_macro2::Ident;
    use syn::{ Data, DataStruct, Error, Fields, FieldsNamed };

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

    let first_field = derive_input.extract_first()?;
    let first_ident: &Ident = first_field.extract()?;
    let first_ident = first_ident.to_string();

    let last_field = derive_input.extract_last()?;
    let last_ident: &Ident = last_field.extract()?;
    let last_ident = last_ident.to_string();

    let nth_0_field = derive_input.extract_nth(0)?;
    let nth_0_ident: &Ident = nth_0_field.extract()?;
    let nth_0_ident = nth_0_ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_extract_derive_input_first_field_name() {
                assert_eq!(#first_ident, #first_field_name);
            }

            #[test]
            fn can_extract_derive_input_last_field_name() {
                assert_eq!(#last_ident, #last_field_name);
            }

            #[test]
            fn can_extract_derive_input_0th_field_name() {
                assert_eq!(#nth_0_ident, #first_field_name);
            }
        }
    )
}

#[cfg(feature = "testsuite")]
pub fn quote_second_named_field_ident(derive_input: &DeriveInput) -> Result<TokenStream> {
    use proc_macro2::Ident;
    use syn::{ Data, DataStruct, Error, Fields, FieldsNamed };

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

    let field = derive_input.extract_nth(1)?;
    let ident: &Ident = field.extract()?;
    let ident = ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_extract_derive_input_second_field_name() {
                assert_eq!(#ident, #field_name);
            }
        }
    )
}