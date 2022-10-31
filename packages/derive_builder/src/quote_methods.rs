use proc_macro2::{ TokenStream, Ident };
use quote::quote;

pub fn quote_methods(field_name: &Ident, method_name: Option<&Ident>, is_vec: bool) -> TokenStream {
    if let Some(method_name) = method_name {
        let conflicting_names = field_name == method_name;

        if conflicting_names {
            quote_global_method(field_name, method_name)
        } else {
            quote_both_methods(field_name, method_name)
        }
    } else {
        let type_name = if is_vec {
            quote! { Vec<String> }
        } else {
            quote! { String }
        };

        quote_individual_method(field_name, type_name)
    }
}

#[inline]
fn quote_individual_method(field_name: &Ident, type_name: TokenStream) -> TokenStream {
    quote! {
        fn #field_name(&mut self, #field_name: #type_name) -> &mut Self {
            self.#field_name = Some(#field_name);
            self
        }
    }
}

#[inline]
fn quote_global_method(field_name: &Ident, method_name: &Ident) -> TokenStream {
    quote! {
        fn #method_name(&mut self, #field_name: String) -> &mut Self {
            if let Some(ref mut field_name) = self.#field_name {
                field_name.push(#field_name);
            } else {
                self.#field_name = Some(vec![#field_name]);
            }

            self
        }
    }
}

// args and arg, but not conflicting env and env
#[inline]
fn quote_both_methods(field_name: &Ident, method_name: &Ident) -> TokenStream {
    let quote_global_method = quote_global_method(field_name, method_name);
    let quote_individual_method: TokenStream = quote_individual_method(
        field_name,
        quote! { Vec<String> }
    );

    quote! {
        #quote_global_method
        #quote_individual_method
    }
}