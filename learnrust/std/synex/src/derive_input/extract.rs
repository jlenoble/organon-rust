use proc_macro2::Ident;
use syn::{
    Data,
    DataStruct,
    Error,
    DeriveInput,
    Field,
    Fields,
    Result,
    FieldsNamed,
    punctuated::Punctuated,
    token::Comma,
};

#[cfg(feature = "testsuite")]
use proc_macro2::TokenStream;
#[cfg(feature = "testsuite")]
use quote::quote;

use crate::Extract;

impl Extract<Ident> for DeriveInput {
    fn extract(&self) -> Result<&Ident> {
        Ok(&self.ident)
    }
}

#[cfg(feature = "testsuite")]
pub fn quote_ident(derive_input: &DeriveInput) -> Result<TokenStream> {
    let input_name = derive_input.ident.to_string();
    let ident: &Ident = derive_input.extract()?;
    let ident = ident.to_string();

    Ok(
        quote! {
            #[test]
            fn can_extract_derive_input_name() {
                assert_eq!(#ident, #input_name);
            }
        }
    )
}

impl Extract<Fields> for DeriveInput {
    fn extract(&self) -> Result<&Fields> {
        match self.data {
            Data::Struct(DataStruct { struct_token: _, ref fields, semi_token: _ }) => {
                Ok(fields)
            }
            Data::Enum(_) => {
                Err(Error::new_spanned(self, "expected Struct as Data in DeriveInput, got Enum"))
            }
            Data::Union(_) => {
                Err(Error::new_spanned(self, "expected Struct as Data in DeriveInput, got Union"))
            }
        }
    }
}

impl Extract<FieldsNamed> for DeriveInput {
    fn extract(&self) -> Result<&FieldsNamed> {
        Fields::extract(DeriveInput::extract(self)?)
    }
}

impl Extract<Punctuated<Field, Comma>> for DeriveInput {
    fn extract(&self) -> Result<&Punctuated<Field, Comma>> {
        FieldsNamed::extract(DeriveInput::extract(self)?)
    }
}

impl Extract<Field> for DeriveInput {
    fn extract(&self) -> Result<&Field> {
        FieldsNamed::extract(DeriveInput::extract(self)?)
    }
}

#[cfg(feature = "testsuite")]
pub fn quote_only_one_named_field_ident(derive_input: &DeriveInput) -> Result<TokenStream> {
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

    let field: &Field = derive_input.extract()?;
    let ident: &Ident = field.extract()?;
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