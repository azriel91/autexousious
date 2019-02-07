use std::collections::HashMap;

use amethyst::{renderer::SpriteRender, Error};
use collision_loading::InteractionAnimationLoader;
use collision_model::{animation::InteractionFrameActiveHandle, config::InteractionFrame};
use fnv::FnvHashMap;
use game_model::config::AssetRecord;
use log::debug;
use object_model::{
    config::ObjectDefinition,
    loaded::{
        AnimatedComponentAnimation, AnimatedComponentDefault, GameObject, Object, ObjectWrapper,
        SequenceEndTransition, SequenceEndTransitions,
    },
};
use sprite_loading::{SpriteLoader, SpriteRenderAnimationLoader};

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
    /// * `asset_record`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<O>(
        ObjectLoaderParams {
            loader,
            texture_assets,
            sprite_sheet_assets,
            sprite_render_primitive_sampler_assets,
            sprite_render_animation_assets,
            interaction_frame_assets,
            interaction_frame_primitive_sampler_assets,
            interaction_frame_animation_assets,
        }: ObjectLoaderParams,
        asset_record: &AssetRecord,
        object_definition: &ObjectDefinition<O::SequenceId>,
    ) -> Result<(O::ObjectWrapper, SequenceEndTransitions<O::SequenceId>), Error>
    where
        O: GameObject,
    {
        debug!("Loading object assets in `{}`", asset_record.path.display());

        let sequence_end_transitions = object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (*sequence_id, SequenceEndTransition::new(sequence.next))
            })
            .collect::<FnvHashMap<_, _>>();

        let (sprite_sheet_handles, _texture_handles) = SpriteLoader::load(
            loader,
            texture_assets,
            sprite_sheet_assets,
            &asset_record.path,
        )?;

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

            let interaction_frame_handle = {
                loader.load_from_data(InteractionFrame::default(), (), interaction_frame_assets)
            };
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
        let object = Object::new(animation_defaults, animations, component_sequences);
        let wrapper = O::ObjectWrapper::new(object);

        Ok((wrapper, sequence_end_transitions.into()))
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        animation::{Animation, AnimationBundle, Sampler, SpriteRenderPrimitive},
        assets::{AssetStorage, Loader},
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
            BodyFrameActiveHandle, InteractionFrameActiveHandle, InteractionFramePrimitive,
        },
        config::InteractionFrame,
    };
    use game_model::config::AssetRecord;

    use super::ObjectLoader;
    use crate::{object::object_loader_params::ObjectLoaderParams, ObjectLoadingBundle};

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
                .with_bundle(ObjectLoadingBundle::new())
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

                    let loader = &world.read_resource::<Loader>();
                    let texture_assets = &world.read_resource::<AssetStorage<Texture>>();
                    let sprite_sheet_assets = &world.read_resource::<AssetStorage<SpriteSheet>>();
                    let sprite_render_primitive_sampler_assets =
                        &world.read_resource::<AssetStorage<Sampler<SpriteRenderPrimitive>>>();
                    let sprite_render_animation_assets =
                        &world.read_resource::<AssetStorage<Animation<SpriteRender>>>();
                    let interaction_frame_assets =
                        &world.read_resource::<AssetStorage<InteractionFrame>>();
                    let interaction_frame_primitive_sampler_assets =
                        &world.read_resource::<AssetStorage<Sampler<InteractionFramePrimitive>>>();
                    let interaction_frame_animation_assets = &world
                        .read_resource::<AssetStorage<Animation<InteractionFrameActiveHandle>>>();

                    let (object, _sequence_end_transitions) = ObjectLoader::load::<Character>(
                        ObjectLoaderParams {
                            loader,
                            texture_assets,
                            sprite_sheet_assets,
                            sprite_render_primitive_sampler_assets,
                            sprite_render_animation_assets,
                            interaction_frame_assets,
                            interaction_frame_primitive_sampler_assets,
                            interaction_frame_animation_assets,
                        },
                        &asset_record,
                        &character_definition.object_definition,
                    )
                    .expect("Failed to load object");

                    world.add_resource(object);
                })
                .with_assertion(|world| {
                    let object = world.read_resource::<CharacterObjectWrapper>();

                    // See bat/object.toml
                    assert_eq!(16, object.animations.len());

                    let stand_attack_animations = object
                        .animations
                        .get(&CharacterSequenceId::StandAttack)
                        .expect("Expected to read `StandAttack` animations.");
                    // InteractionFrame and SpriteRender
                    assert_eq!(2, stand_attack_animations.len());
                })
                .run()
                .is_ok()
        );
    }
}
