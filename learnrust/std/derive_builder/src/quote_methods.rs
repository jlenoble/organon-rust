use proc_macro2::{ TokenStream, Ident };
use quote::{ quote, format_ident };
use syn::{ Error, Field, Fields, Result, TypePath, MetaList, MetaNameValue };

use synex::{ Extract, ExtractIter, ExtractValue };

pub fn quote_methods(fields: &Fields) -> Result<TokenStream> {
    let mut quote_each_method: Vec<TokenStream> = Vec::new();

    for field in fields.extract_iter()? {
        quote_each_method.push(quote_method(field)?);
    }

    Ok(quote! { #(#quote_each_method)* })
}

fn quote_method(field: &Field) -> Result<TokenStream> {
    let field_name: &Ident = field.extract()?;

    let typepath: &TypePath = field.extract()?;
    let type_name: &Ident = typepath.extract()?;

    let meta = field.extract_iter()?.next();

    if meta.is_none() {
        let type_name = if type_name == "Vec" {
            quote! { ::std::vec::Vec<::std::string::String> }
        } else {
            quote! { ::std::string::String }
        };

        Ok(quote_eponymous_method_only(field_name, type_name))
    } else {
        let meta = meta.unwrap()?;

        let list: &MetaList = meta.extract()?;
        let ident: &Ident = list.extract()?;
        let MetaNameValue { path, lit, eq_token: _ } = list.extract()?;

        let name: &Ident = path.extract()?;

        if ident != "builder" || name != "each" {
            return Err(Error::new_spanned(meta, "expected `builder(each = \"...\")`"));
        }

        let ref method_name: Ident = format_ident!("{}", lit.extract_value()?);

        if field_name == method_name {
            Ok(quote_inert_method_only(field_name, method_name))
        } else {
            Ok(quote_both_methods(field_name, method_name))
        }
    }
}

#[inline]
fn quote_eponymous_method_only(field_name: &Ident, type_name: TokenStream) -> TokenStream {
    quote! {
        fn #field_name(&mut self, #field_name: #type_name) -> &mut Self {
            self.#field_name = ::core::option::Option::Some(#field_name);
            self
        }
    }
}

#[inline]
fn quote_inert_method_only(field_name: &Ident, method_name: &Ident) -> TokenStream {
    quote! {
        fn #method_name(&mut self, #field_name: ::std::string::String) -> &mut Self {
            if let ::core::option::Option::Some(ref mut field_name) = self.#field_name {
                field_name.push(#field_name);
            } else {
                self.#field_name = ::core::option::Option::Some(::std::vec![#field_name]);
            }

            self
        }
    }
}

#[inline]
fn quote_both_methods(field_name: &Ident, method_name: &Ident) -> TokenStream {
    let quote_inert_method = quote_inert_method_only(field_name, method_name);
    let quote_eponymous_method: TokenStream = quote_eponymous_method_only(
        field_name,
        quote! { ::std::vec::Vec<::std::string::String> }
    );

    quote! {
        #quote_inert_method
        #quote_eponymous_method
    }
}