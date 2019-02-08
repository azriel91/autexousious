use std::collections::HashMap;

use amethyst::{renderer::SpriteRender, Error};
use collision_loading::{BodyAnimationLoader, InteractionAnimationLoader};
use collision_model::{
    animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle},
    config::{BodyFrame, InteractionFrame},
};
use fnv::FnvHashMap;
use object_model::{
    config::ObjectDefinition,
    loaded::{
        AnimatedComponentAnimation, AnimatedComponentDefault, GameObject, Object, ObjectWrapper,
        SequenceEndTransition,
    },
};
use sprite_loading::SpriteRenderAnimationLoader;

use crate::ObjectLoaderParams;

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug)]
pub struct ObjectLoader;

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `object_loader_params`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<O>(
        ObjectLoaderParams {
            loader,
            sprite_sheet_handles,
            sprite_render_primitive_sampler_assets,
            sprite_render_animation_assets,
            body_frame_assets,
            body_frame_primitive_sampler_assets,
            body_frame_animation_assets,
            interaction_frame_assets,
            interaction_frame_primitive_sampler_assets,
            interaction_frame_animation_assets,
        }: ObjectLoaderParams,
        object_definition: &ObjectDefinition<O::SequenceId>,
    ) -> Result<O::ObjectWrapper, Error>
    where
        O: GameObject,
    {
        let sequence_end_transitions = object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (*sequence_id, SequenceEndTransition::new(sequence.next))
            })
            .collect::<FnvHashMap<_, _>>();

        // === Animation Component Defaults === //

        // Load the animation defaults in a separate scope because the animations' loaders may read
        // the `AnimationDataSet`s mutably, and that will cause a panic at runtime since loading
        // animation defaults borrows them immutably.

        let sprite_sheet = sprite_sheet_handles
            .iter()
            .next()
            .expect("Expected character to have at least one sprite sheet.")
            .clone();

        let animation_defaults = {
            let mut animation_defaults = Vec::new();

            let sprite_render = SpriteRender {
                sprite_sheet,
                sprite_number: 0,
            };
            animation_defaults.push(AnimatedComponentDefault::SpriteRender(sprite_render));

            let body_frame_handle =
                loader.load_from_data(BodyFrame::default(), (), body_frame_assets);
            let body_frame_active_handle = BodyFrameActiveHandle::new(body_frame_handle);
            animation_defaults.push(AnimatedComponentDefault::BodyFrame(
                body_frame_active_handle,
            ));

            let interaction_frame_handle =
                loader.load_from_data(InteractionFrame::default(), (), interaction_frame_assets);
            let interaction_frame_active_handle =
                InteractionFrameActiveHandle::new(interaction_frame_handle);
            animation_defaults.push(AnimatedComponentDefault::InteractionFrame(
                interaction_frame_active_handle,
            ));

            animation_defaults
        };

        // === Animations === //

        let mut sprite_render_animations = SpriteRenderAnimationLoader::load_into_map(
            loader,
            sprite_render_primitive_sampler_assets,
            sprite_render_animation_assets,
            &object_definition.sequences,
            &sprite_sheet_handles,
        );
        let mut body_frame_animations = BodyAnimationLoader::load_into_map(
            loader,
            body_frame_assets,
            body_frame_primitive_sampler_assets,
            body_frame_animation_assets,
            &object_definition.sequences,
        );
        let mut interaction_frame_animations = InteractionAnimationLoader::load_into_map(
            loader,
            interaction_frame_assets,
            interaction_frame_primitive_sampler_assets,
            interaction_frame_animation_assets,
            &object_definition.sequences,
        );

        let animations = object_definition
            .sequences
            .keys()
            .map(move |sequence_id| {
                let mut animations = Vec::new();
                if let Some(sprite_render) = sprite_render_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::SpriteRender(sprite_render));
                }
                if let Some(body_frame) = body_frame_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::BodyFrame(body_frame));
                }
                if let Some(interaction_frame) = interaction_frame_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::InteractionFrame(
                        interaction_frame,
                    ));
                }

                (*sequence_id, animations)
            })
            .collect::<HashMap<_, _>>();

        // TODO: implement -- load component sequences
        let component_sequences = HashMap::new();

        let object = Object::new(
            animation_defaults,
            animations,
            component_sequences,
            sequence_end_transitions.into(),
        );
        let wrapper = O::ObjectWrapper::new(object);

        Ok(wrapper)
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        animation::{Animation, AnimationBundle, Sampler, SpriteRenderPrimitive},
        assets::{AssetStorage, Loader, Processor},
        renderer::{SpriteRender, SpriteSheet, Texture},
    };
    use amethyst_test::AmethystApplication;
    use application::{load_in, Format};
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use character_model::{
        config::{CharacterDefinition, CharacterSequenceId},
        loaded::{Character, CharacterObjectWrapper},
    };
    use collision_loading::CollisionLoadingBundle;
    use collision_model::{
        animation::{
            BodyFrameActiveHandle, BodyFramePrimitive, InteractionFrameActiveHandle,
            InteractionFramePrimitive,
        },
        config::{BodyFrame, InteractionFrame},
    };
    use game_model::config::AssetRecord;
    use sprite_loading::SpriteLoader;
    use sprite_model::config::SpritesDefinition;
    use typename::TypeName;

    use super::ObjectLoader;
    use crate::{
        object::object_loader_params::ObjectLoaderParams, ObjectDefinitionToWrapperProcessor,
    };

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_object_assets", false)
                .with_bundle(
                    AnimationBundle::<CharacterSequenceId, BodyFrameActiveHandle>::new(
                        "character_body_frame_acs",
                        "character_body_frame_sis",
                    )
                )
                .with_bundle(AnimationBundle::<
                    CharacterSequenceId,
                    InteractionFrameActiveHandle,
                >::new(
                    "character_interaction_frame_acs",
                    "character_interaction_frame_sis",
                ))
                .with_bundle(CollisionLoadingBundle::new())
                .with_system(
                    ObjectDefinitionToWrapperProcessor::<Character>::new(),
                    ObjectDefinitionToWrapperProcessor::<Character>::type_name(),
                    &[]
                )
                .with_system(Processor::<Character>::new(), "character_processor", &[])
                .with_effect(|world| {
                    let asset_record = AssetRecord::new(
                        ASSETS_CHAR_BAT_SLUG.clone(),
                        ASSETS_CHAR_BAT_PATH.clone(),
                    );

                    let character_definition = load_in::<CharacterDefinition, _>(
                        &asset_record.path,
                        "object.toml",
                        Format::Toml,
                        None,
                    )
                    .expect("Failed to load object.toml into CharacterDefinition");

                    let object_wrapper = {
                        let sprites_definition = load_in::<SpritesDefinition, _>(
                            &asset_record.path,
                            "sprites.toml",
                            Format::Toml,
                            None,
                        )
                        .expect("Failed to load sprites_definition.");

                        let loader = &world.read_resource::<Loader>();
                        let texture_assets = &world.read_resource::<AssetStorage<Texture>>();
                        let sprite_sheet_assets =
                            &world.read_resource::<AssetStorage<SpriteSheet>>();

                        let sprite_render_primitive_sampler_assets =
                            &world.read_resource::<AssetStorage<Sampler<SpriteRenderPrimitive>>>();
                        let sprite_render_animation_assets =
                            &world.read_resource::<AssetStorage<Animation<SpriteRender>>>();
                        let body_frame_assets = &world.read_resource::<AssetStorage<BodyFrame>>();
                        let body_frame_primitive_sampler_assets =
                            &world.read_resource::<AssetStorage<Sampler<BodyFramePrimitive>>>();
                        let body_frame_animation_assets = &world
                            .read_resource::<AssetStorage<Animation<BodyFrameActiveHandle>>>();
                        let interaction_frame_assets =
                            &world.read_resource::<AssetStorage<InteractionFrame>>();
                        let interaction_frame_primitive_sampler_assets = &world
                            .read_resource::<AssetStorage<Sampler<InteractionFramePrimitive>>>();
                        let interaction_frame_animation_assets = &world
                            .read_resource::<AssetStorage<Animation<InteractionFrameActiveHandle>>>(
                            );

                        // TODO: <https://gitlab.com/azriel91/autexousious/issues/94>
                        let sprite_sheet_handles = SpriteLoader::load(
                            loader,
                            texture_assets,
                            sprite_sheet_assets,
                            &sprites_definition,
                            &asset_record.path,
                        )
                        .expect("Failed to load sprites.");
                        let sprite_sheet_handles = &sprite_sheet_handles;

                        ObjectLoader::load::<Character>(
                            ObjectLoaderParams {
                                loader,
                                sprite_sheet_handles,
                                sprite_render_primitive_sampler_assets,
                                sprite_render_animation_assets,
                                body_frame_assets,
                                body_frame_primitive_sampler_assets,
                                body_frame_animation_assets,
                                interaction_frame_assets,
                                interaction_frame_primitive_sampler_assets,
                                interaction_frame_animation_assets,
                            },
                            &character_definition.object_definition,
                        )
                        .expect("Failed to load object")
                    };

                    world.add_resource(object_wrapper);
                })
                .with_assertion(|world| {
                    let object_wrapper = world.read_resource::<CharacterObjectWrapper>();

                    // See bat/object.toml
                    assert_eq!(16, object_wrapper.animations.len());

                    let stand_attack_animations = object_wrapper
                        .animations
                        .get(&CharacterSequenceId::StandAttack)
                        .expect("Expected to read `StandAttack` animations.");
                    // SpriteRender, BodyFrame, and InteractionFrame
                    assert_eq!(3, stand_attack_animations.len());
                })
                .run()
                .is_ok()
        );
    }
}
