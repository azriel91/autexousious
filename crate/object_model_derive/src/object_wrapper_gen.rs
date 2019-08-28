use quote::quote;
use syn::{Ident, Path, Visibility};

/// Generates the newtype implementation for the `ObjectWrapper`.
///
/// See `object_model::loaded::ObjectWrapper`.
pub fn object_wrapper_gen(
    object_definition_type: &Path,
    object_wrapper_name: &Ident,
    vis: &Visibility,
) -> proc_macro2::TokenStream {
    // TODO: Trait delegation pending <https://github.com/rust-lang/rfcs/pull/2393>
    let doc_string = "Newtype for `Object`.";

    let doc_fn_new = format!("Returns a new {}", object_wrapper_name);

    quote! {
        #[doc = #doc_string]
        #[derive(Clone, Debug, PartialEq, typename_derive::TypeName)]
        #vis struct #object_wrapper_name(#vis object_model::loaded::Object);

        impl #object_wrapper_name {
            #[doc = #doc_fn_new]
            pub fn new(object: object_model::loaded::Object) -> Self {
                #object_wrapper_name(object)
            }
        }

        impl std::ops::Deref for #object_wrapper_name {
            type Target = object_model::loaded::Object;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for #object_wrapper_name {
            fn deref_mut(&mut self) -> &mut object_model::loaded::Object {
                &mut self.0
            }
        }

        impl object_model::loaded::ObjectWrapper for #object_wrapper_name {
            fn new(object: object_model::loaded::Object) -> Self {
                #object_wrapper_name::new(object)
            }

            fn inner(&self) -> &object_model::loaded::Object {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut object_model::loaded::Object {
                &mut self.0
            }
        }

        impl amethyst::assets::Asset for #object_wrapper_name {
            type Data = object_model::config::ObjectAssetData<#object_definition_type>;
            type HandleStorage = amethyst::ecs::storage::VecStorage<amethyst::assets::Handle<Self>>;

            const NAME: &'static str =
                concat!(module_path!(), "::", stringify!(#object_wrapper_name));
        }

        impl std::convert::AsRef<sequence_model::loaded::WaitSequenceHandles>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &sequence_model::loaded::WaitSequenceHandles {
                &self.0.wait_sequence_handles
            }
        }

        impl std::convert::AsRef<
            sprite_model::loaded::SpriteRenderSequenceHandles
        > for #object_wrapper_name
        {
            fn as_ref(&self) -> &sprite_model::loaded::SpriteRenderSequenceHandles
            {
                &self.0.sprite_render_sequence_handles
            }
        }

        impl std::convert::AsRef<collision_model::loaded::BodySequenceHandles>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &collision_model::loaded::BodySequenceHandles {
                &self.0.body_sequence_handles
            }
        }

        impl std::convert::AsRef<
            collision_model::loaded::InteractionsSequenceHandles
        > for #object_wrapper_name
        {
            fn as_ref(&self) -> &collision_model::loaded::InteractionsSequenceHandles
            {
                &self.0.interactions_sequence_handles
            }
        }

        impl std::convert::AsRef<spawn_model::loaded::SpawnsSequenceHandles>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &spawn_model::loaded::SpawnsSequenceHandles {
                &self.0.spawns_sequence_handles
            }
        }

        impl std::convert::AsRef<sequence_model::loaded::SequenceEndTransitions>
        for #object_wrapper_name
        {
            fn as_ref(&self) -> &sequence_model::loaded::SequenceEndTransitions {
                &self.0.sequence_end_transitions
            }
        }
    }
}
