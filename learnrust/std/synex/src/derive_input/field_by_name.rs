use syn::{ DeriveInput, Field, FieldsNamed, Result };

#[cfg(feature = "testsuite")]
use proc_macro2::TokenStream;
#[cfg(feature = "testsuite")]
use quote::quote;

use crate::{ Extract, FieldByName };

impl FieldByName for DeriveInput {
    fn field_by_name<'b>(&'b self, name: &'b str) -> Result<&'b Field> {
        FieldsNamed::field_by_name(self.extract()?, name)
    }
}

#[cfg(feature = "testsuite")]
pub fn quote_find_field_by_name(derive_input: &DeriveInput) -> Result<TokenStream> {
    use proc_macro2::Ident;
    use syn::{ Data, DataStruct, Error, Fields };

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

    let first_field = derive_input.field_by_name(first_field_name.as_str())?;
    let first_ident: &Ident = first_field.extract()?;
    let first_ident = first_ident.to_string();

    let last_field = derive_input.field_by_name(last_field_name.as_str())?;
    let last_ident: &Ident = last_field.extract()?;
    let last_ident = last_ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_find_derive_input_field_by_field_name() {
                assert_eq!(#first_ident, #first_field_name);
                assert_eq!(#last_ident, #last_field_name);
            }
        }
    )
}