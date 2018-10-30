use std::collections::HashMap;

use amethyst::{
    prelude::*,
    renderer::{SpriteRender, SpriteSheetSet},
};
use application::Result;
use collision_loading::CollisionAnimationLoader;
use collision_model::animation::{
    CollisionDataSet, CollisionFrameActiveHandle, DEFAULT_COLLISION_FRAME_ID,
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
        let sprite_sheet_index_offset = world.read_resource::<SpriteSheetSet>().len() as u64;
        let collision_frame_offset = world.read_resource::<CollisionDataSet>().len() as u64;

        debug!("Loading object assets in `{}`", asset_record.path.display());

        let (sprite_sheet_handles, _texture_handles) =
            SpriteLoader::load(world, sprite_sheet_index_offset, &asset_record.path)?;
        let sprite_sheet = sprite_sheet_handles
            .into_iter()
            .next()
            .expect("Expected character to have at least one sprite sheet.");

        // === Animation Component Defaults === //

        // Load the animation defaults in a separate scope because the animations' loaders may read
        // the `AnimationDataSet`s mutably, and that will cause a panic at runtime since loading
        // animation defaults borrows them immutably.
        let animation_defaults = {
            let mut animation_defaults = Vec::new();
            let collision_data_set = world.read_resource::<CollisionDataSet>();

            let sprite_render = SpriteRender {
                sprite_sheet,
                sprite_number: 0,
                flip_horizontal: false,
                flip_vertical: false,
            };
            animation_defaults.push(AnimatedComponentDefault::SpriteRender(sprite_render));

            let collision_frame_handle = collision_data_set
                .data(DEFAULT_COLLISION_FRAME_ID)
                .expect("Expected default collision frame to be loaded.");
            let collision_frame_active_handle =
                CollisionFrameActiveHandle::new(collision_frame_handle);
            animation_defaults.push(AnimatedComponentDefault::CollisionFrame(
                collision_frame_active_handle,
            ));

            animation_defaults
        };

        // === Animations === //

        let mut sprite_render_animations = SpriteRenderAnimationLoader::load_into_map(
            world,
            &object_definition.sequences,
            sprite_sheet_index_offset,
        );
        let mut collision_frame_animations = CollisionAnimationLoader::load_into_map(
            world,
            &object_definition.sequences,
            collision_frame_offset,
        );

        let animations = object_definition
            .sequences
            .keys()
            .map(move |sequence_id| {
                let mut animations = Vec::new();
                if let Some(sprite_render) = sprite_render_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::SpriteRender(sprite_render));
                }
                if let Some(collision_frame) = collision_frame_animations.remove(sequence_id) {
                    animations.push(AnimatedComponentAnimation::CollisionFrame(collision_frame));
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
    use amethyst_test_support::AmethystApplication;
    use application::{load_in, Format};
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use collision_loading::CollisionLoadingBundle;
    use collision_model::animation::CollisionFrameActiveHandle;
    use game_model::config::AssetRecord;
    use object_model::config::{object::CharacterSequenceId, CharacterDefinition};

    use super::ObjectLoader;

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_object_assets", false)
                .with_bundle(AnimationBundle::<
                    CharacterSequenceId,
                    CollisionFrameActiveHandle,
                >::new(
                    "character_collision_frame_acs",
                    "character_collision_frame_sis",
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
                })
                .run()
                .is_ok()
        );
    }
}
