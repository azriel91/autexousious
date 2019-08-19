use quote::{quote, ToTokens};
use syn::{Ident, Path, Visibility};

/// Generates the newtype implementation for the `ObjectWrapper`.
///
/// See `object_model::loaded::ObjectWrapper`.
pub fn object_wrapper_gen(
    sequence_id_type: &Path,
    object_definition_type: &Path,
    object_wrapper_name: &Ident,
    vis: &Visibility,
) -> proc_macro2::TokenStream {
    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    let doc_string = {
        let path_segment = sequence_id_type
            .segments
            .last()
            .expect("Failed to get last path segment for Sequence ID type.");

        format!(
            "Newtype for `Object<{}>`.",
            path_segment.clone().into_token_stream()
        )
    };

    let doc_fn_new = format!("Returns a new {}", object_wrapper_name);

    quote! {
        #[doc = #doc_string]
        #[derive(Clone, Debug, PartialEq, typename_derive::TypeName)]
        #vis struct #object_wrapper_name(#vis object_model::loaded::Object<#sequence_id_type>);

        impl #object_wrapper_name {
            #[doc = #doc_fn_new]
            pub fn new(object: object_model::loaded::Object<#sequence_id_type>) -> Self {
                #object_wrapper_name(object)
            }
        }

        impl std::ops::Deref for #object_wrapper_name {
            type Target = object_model::loaded::Object<#sequence_id_type>;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #object_wrapper_name {
            fn deref_mut(&mut self) -> &mut object_model::loaded::Object<#sequence_id_type> {
                &mut self.0
            }
        }

        impl object_model::loaded::ObjectWrapper for #object_wrapper_name {
            type SequenceId = #sequence_id_type;

            fn new(object: object_model::loaded::Object<Self::SequenceId>) -> Self {
                #object_wrapper_name::new(object)
            }

            fn inner(&self) -> &object_model::loaded::Object<Self::SequenceId> {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut object_model::loaded::Object<Self::SequenceId> {
                &mut self.0
            }
        }

        impl amethyst::assets::Asset for #object_wrapper_name {
            const NAME: &'static str = concat!(
                module_path!(),
                "::",
                stringify!(#object_wrapper_name),
            );

            type Data = object_model::config::ObjectAssetData<#object_definition_type>;
            type HandleStorage = amethyst::ecs::storage::VecStorage<amethyst::assets::Handle<Self>>;
        }

        impl std::convert::AsRef<sequence_model::loaded::WaitSequenceHandles<#sequence_id_type>>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &sequence_model::loaded::WaitSequenceHandles<#sequence_id_type> {
                &self.0.wait_sequence_handles
            }
        }

        impl std::convert::AsRef<
            sprite_model::loaded::SpriteRenderSequenceHandles<#sequence_id_type>
        > for #object_wrapper_name
        {
            fn as_ref(&self) -> &sprite_model::loaded::SpriteRenderSequenceHandles<
                #sequence_id_type
            >
            {
                &self.0.sprite_render_sequence_handles
            }
        }

        impl std::convert::AsRef<collision_model::loaded::BodySequenceHandles<#sequence_id_type>>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &collision_model::loaded::BodySequenceHandles<#sequence_id_type> {
                &self.0.body_sequence_handles
            }
        }

        impl std::convert::AsRef<
            collision_model::loaded::InteractionsSequenceHandles<#sequence_id_type>
        > for #object_wrapper_name
        {
            fn as_ref(&self) -> &collision_model::loaded::InteractionsSequenceHandles<
                #sequence_id_type
            >
            {
                &self.0.interactions_sequence_handles
            }
        }

        impl std::convert::AsRef<spawn_model::loaded::SpawnsSequenceHandles<#sequence_id_type>>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &spawn_model::loaded::SpawnsSequenceHandles<#sequence_id_type> {
                &self.0.spawns_sequence_handles
            }
        }

        impl std::convert::AsRef<sequence_model::loaded::SequenceEndTransitions<#sequence_id_type>>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &sequence_model::loaded::SequenceEndTransitions<#sequence_id_type> {
                &self.0.sequence_end_transitions
            }
        }
    }
}
