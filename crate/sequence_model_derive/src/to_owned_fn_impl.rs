use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Path};

/// Returns an function implementation for the `ComponentData::to_owned` trait method.
pub fn to_owned_fn_impl(component_copy: bool, to_owned_fn: Option<Path>) -> TokenStream {
    if component_copy {
        quote! {
            fn to_owned(component: &Self::Component) -> Self::Component {
                *component
            }
        }
    } else {
        let to_owned_fn = to_owned_fn.unwrap_or_else(|| parse_quote!(std::clone::Clone::clone));

        quote! {
            fn to_owned(component: &Self::Component) -> Self::Component {
                #to_owned_fn(component)
            }
        }
    }
}
