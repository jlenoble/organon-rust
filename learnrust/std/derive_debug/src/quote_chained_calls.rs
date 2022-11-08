use proc_macro2::{ TokenStream, Ident };
use quote::quote;
use syn::{ Error, Field, FieldsNamed, Meta, MetaNameValue, Result };

use synex::{ Extract, ExtractIter, ExtractValue };

pub fn quote_chained_calls(fields: &FieldsNamed) -> Result<TokenStream> {
    let mut quote_each_call: Vec<TokenStream> = Vec::new();

    for field in fields.extract_iter()? {
        quote_each_call.push(quote_chained_call(field)?);
    }

    Ok(quote! { #(#quote_each_call)* })
}

fn quote_chained_call(field: &Field) -> Result<TokenStream> {
    let field_name: &Ident = field.extract()?;
    let field_name_as_string = field_name.to_string();

    let meta = <&Field as ExtractIter<'_, Meta>>::extract_iter(&field)?.next();

    if meta.is_none() {
        Ok(quote! {
            .field(#field_name_as_string, &self.#field_name)
        })
    } else {
        let meta = meta.unwrap()?;

        let MetaNameValue { path, lit, eq_token: _ } = meta.extract()?;

        let name: &Ident = path.extract()?;

        if name != "debug" {
            return Err(Error::new_spanned(meta, "expected `debug = \"...\"`"));
        }

        let custom_format = lit.extract_value()?;

        Ok(
            quote! {
                .field(#field_name_as_string, &::std::format_args!(#custom_format, &self.#field_name))
            }
        )
    }
}