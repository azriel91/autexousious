use quote::{quote, ToTokens};
use syn::{punctuated::Pair, Ident, Path, Visibility};

/// Generates the newtype implementation for the `ObjectWrapper`.
///
/// See `object_model::loaded::ObjectWrapper`.
pub fn object_wrapper_gen(
    sequence_id_type: &Path,
    object_wrapper_type: &Ident,
    vis: &Visibility,
) -> proc_macro2::TokenStream {
    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    let doc_string = {
        let last_segment = sequence_id_type
            .segments
            .last()
            .expect("Failed to get last path segment for Sequence ID type.");
        if let Pair::End(path_segment) = last_segment {
            format!(
                "Newtype for `Object<{}>`.",
                path_segment.clone().into_token_stream()
            )
        } else {
            String::from("Newtype for `Object<?>`.")
        }
    };

    quote! {
        use derive_deref::{Deref, DerefMut};

        #[doc = #doc_string]
        #[derive(Debug, Deref, DerefMut)]
        #vis struct #object_wrapper_type(#vis object_model::loaded::ObjectHandle<#sequence_id_type>);

        impl object_model::loaded::ObjectWrapper for #object_wrapper_type {
            type SequenceId = #sequence_id_type;

            fn new(object_handle: object_model::loaded::ObjectHandle<Self::SequenceId>) -> Self {
                #object_wrapper_type(object_handle)
            }
        }

        impl amethyst::assets::Asset for #object_wrapper_type {
            const NAME: &'static str = concat!(
                module_path!(),
                "::",
                stringify!(#object_wrapper_type)
            );

            type Data = Self;
            type HandleStorage = amethyst::ecs::storage::VecStorage<amethyst::assets::Handle<Self>>;
        }

        impl From<#object_wrapper_type> for std::result::Result<
            amethyst::assets::ProcessingState<#object_wrapper_type>,
            amethyst::assets::Error
        > {
            fn from(object: #object_wrapper_type) -> std::result::Result<
                amethyst::assets::ProcessingState<#object_wrapper_type>,
                amethyst::assets::Error
            > {
                Ok(amethyst::assets::ProcessingState::Loaded(object))
            }
        }
    }
}
