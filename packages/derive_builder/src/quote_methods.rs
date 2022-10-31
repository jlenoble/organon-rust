use proc_macro2::{ TokenStream, Ident };
use quote::quote;

pub fn quote_all_at_once_setter_method(field_name: &Ident, type_name: TokenStream) -> TokenStream {
    quote! {
        fn #field_name(&mut self, #field_name: #type_name) -> &mut Self {
            self.#field_name = Some(#field_name);
            self
        }
    }
}

pub fn quote_one_at_a_time_setter_method(field_name: &Ident, method_name: &Ident) -> TokenStream {
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
pub fn quote_both_setter_methods(field_name: &Ident, method_name: &Ident) -> TokenStream {
    let quote_global_method = quote_all_at_once_setter_method(field_name, quote! { Vec<String> });
    let quote_individual_method: TokenStream = quote_one_at_a_time_setter_method(
        field_name,
        method_name
    );

    quote! {
        #quote_global_method
        #quote_individual_method
    }
}