use proc_macro2::Ident;
use syn::{ DeriveInput, Field, Result };

#[cfg(feature = "testsuite")]
use proc_macro2::TokenStream;
#[cfg(feature = "testsuite")]
use quote::quote;

use crate::{ Extract, FieldIdent };

impl FieldIdent for DeriveInput {
    fn field_ident(&self) -> Result<&Ident> {
        let field: &Field = self.extract()?;
        field.extract()
    }
}

#[cfg(feature = "testsuite")]
pub fn quote_only_one_named_field_ident(derive_input: &DeriveInput) -> Result<TokenStream> {
    use syn::{ Data, DataStruct, Error, Fields, FieldsNamed };

    let field_name = if
        let Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { ref named, .. }), .. }) =
            derive_input.data
    {
        let field = named.first();

        if let Some(Field { ident: Some(ref field_name), .. }) = field {
            field_name.to_string()
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

    let ident = derive_input.field_ident()?;
    let ident = ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_extract_derive_input_only_field_name() {
                assert_eq!(#ident, #field_name);
            }
        }
    )
}