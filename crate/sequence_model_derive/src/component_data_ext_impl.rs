use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Ident, Path};

/// Returns an function implementation for the `ComponentData::to_owned` trait method.
pub fn component_data_ext_impl(
    type_name: &Ident,
    component_path: &Path,
    component_copy: bool,
    to_owned_fn: Option<Path>,
) -> TokenStream {
    let to_owned_fn_impl = if component_copy {
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
    };

    quote! {
        impl sequence_model_spi::loaded::ComponentDataExt for #type_name {
            type Component = #component_path;

            #to_owned_fn_impl
        }
    }
}
