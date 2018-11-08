use std::collections::HashMap;

use amethyst::{
    assets::Loader,
    prelude::*,
    renderer::{MaterialTextureSet, SpriteRender},
};
use application::Result;
use collision_loading::{BodyAnimationLoader, InteractionAnimationLoader};
use collision_model::{
    animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle},
    config::{BodyFrame, InteractionFrame},
};
use game_model::config::AssetRecord;
use object_model::{
    config::{object::SequenceId, ObjectDefinition},
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault, Object},
};
use sprite_loading::{SpriteLoader, SpriteRenderAnimationLoader};

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug)]
pub struct ObjectLoader;

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the asset record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the object's assets.
    /// * `asset_record`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<SeqId: SequenceId>(
        world: &World,
        asset_record: &AssetRecord,
        object_definition: &ObjectDefinition<SeqId>,
    ) -> Result<Object<SeqId>> {
        let sprite_sheet_index_offset = world.read_resource::<MaterialTextureSet>().len() as u64;

        debug!("Loading object assets in `{}`", asset_record.path.display());

        let (sprite_sheet_handles, _texture_handles) =
            SpriteLoader::load(world, sprite_sheet_index_offset, &asset_record.path)?;

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
                flip_horizontal: false,
                flip_vertical: false,
            };
            animation_defaults.push(AnimatedComponentDefault::SpriteRender(sprite_render));

            let body_frame_handle = {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(BodyFrame::default(), (), &world.read_resource())
            };
            let body_frame_active_handle = BodyFrameActiveHandle::new(body_frame_handle);
            animation_defaults.push(AnimatedComponentDefault::BodyFrame(
                body_frame_active_handle,
            ));

            let interaction_frame_handle = {
                let loader = world.read_resource::<Loader>();
                loader.load_from_data(InteractionFrame::default(), (), &world.read_resource())
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
            world,
            &object_definition.sequences,
            &sprite_sheet_handles,
        );
        let mut body_frame_animations =
            BodyAnimationLoader::load_into_map(world, &object_definition.sequences);
        let mut interaction_frame_animations =
            InteractionAnimationLoader::load_into_map(world, &object_definition.sequences);

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

        Ok(Object::new(animation_defaults, animations))
    }
}

#[cfg(test)]
mod test {
    use amethyst::animation::AnimationBundle;
    use amethyst_test::AmethystApplication;
    use application::{load_in, Format};
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::{BodyFrameActiveHandle, InteractionFrameActiveHandle};
    use game_model::config::AssetRecord;
    use object_model::config::{object::CharacterSequenceId, CharacterDefinition};

    use super::ObjectLoader;

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
                .with_assertion(|world| {
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

                    let object = ObjectLoader::load(
                        world,
                        &asset_record,
                        &character_definition.object_definition,
                    )
                    .expect("Failed to load object");

                    // See bat/object.toml
                    assert_eq!(10, object.animations.len());

                    let stand_attack_animations = object
                        .animations
                        .get(&CharacterSequenceId::StandAttack)
                        .expect("Expected to read `StandAttack` animations.");
                    assert_eq!(3, stand_attack_animations.len());
                })
                .run()
                .is_ok()
        );
    }
}
