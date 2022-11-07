mod generic_param;

mod extract;
mod extract_mut;

mod extract_iter;

mod push_value;

#[macro_export]
macro_rules! define_add_trait_bounds {
    ($($tt:tt)*) => {
        fn add_trait_bounds(mut generics: ::syn::Generics, exclude: ::std::option::Option<::std::collections::HashSet<Ident>>) -> ::syn::Generics {
            if exclude.is_none() {
                for param in &mut generics.params {
                    if let ::syn::GenericParam::Type(ref mut type_param) = *param {
                        type_param.bounds.push(::syn::parse_quote!($($tt)*));
                    }
                }
            } else {
                let exclude = exclude.unwrap();

                for param in &mut generics.params {
                    if let ::syn::GenericParam::Type(ref mut type_param) = *param {
                        if exclude.contains(&type_param.ident) {
                            continue;
                        }

                        type_param.bounds.push(::syn::parse_quote!($($tt)*));
                    }
                }
            }
 
            
            generics
        }
    };
}