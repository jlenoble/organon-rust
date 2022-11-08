use std::collections::HashSet;

use proc_macro;
use proc_macro2::{ TokenStream, Ident };
use quote::quote;
use syn::{ DeriveInput, Error, Field, FieldsNamed, parse_macro_input, Result, TypePath };

use synex::{ Extract, define_add_trait_bounds, ExtractIter };

use crate::quote_chained_calls::quote_chained_calls;

mod quote_chained_calls;

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote_all(&parse_macro_input!(input as DeriveInput)).into()
}

fn quote_all(input: &DeriveInput) -> TokenStream {
    let quote_impl_debug = quote_impl_debug(input).unwrap_or_else(Error::into_compile_error);

    quote! {
        #quote_impl_debug
    }
}

fn quote_impl_debug(input: &DeriveInput) -> Result<TokenStream> {
    let struct_name: &Ident = &input.ident;
    let struct_name_as_string = struct_name.to_string();

    let fields: &FieldsNamed = input.extract()?;
    let mut phantom_data_arguments: HashSet<Ident> = HashSet::new();

    for field in <&FieldsNamed as ExtractIter<'_, Field>>::extract_iter(&fields)? {
        let field_type: &TypePath = match field.extract() {
            Ok(typepath) => typepath,
            Err(_) => {
                continue;
            }
        };
        let type_name: &Ident = field_type.extract()?;

        if type_name == "PhantomData" {
            for ident in <&Field as ExtractIter<'_, Ident>>::extract_iter(&field)? {
                phantom_data_arguments.insert(ident?.clone());
            }
        }
    }

    let generics = add_trait_bounds(input.generics.clone(), Some(phantom_data_arguments));
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let quote_chained_calls = quote_chained_calls(fields)?;

    Ok(
        quote! {
            impl #impl_generics ::std::fmt::Debug for #struct_name #ty_generics #where_clause {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct(#struct_name_as_string)
                        #quote_chained_calls
                        .finish()
                }
            }
        }
    )
}

define_add_trait_bounds!(::std::fmt::Debug);