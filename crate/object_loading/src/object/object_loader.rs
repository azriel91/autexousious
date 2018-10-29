use std::collections::HashMap;

use amethyst::{prelude::*, renderer::SpriteSheetSet};
use application::Result;
use collision_loading::CollisionAnimationLoader;
use collision_model::animation::CollisionDataSet;
use game_model::config::AssetRecord;
use object_model::{
    config::{object::SequenceId, ObjectDefinition},
    loaded::{AnimatedComponentAnimation, Object},
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
        let sprite_sheet_handle = sprite_sheet_handles
            .into_iter()
            .next()
            .expect("Expected character to have at least one sprite sheet.");

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

        Ok(Object::new(sprite_sheet_handle, animations))
    }
}

#[cfg(test)]
mod test {
    use amethyst_test_support::AmethystApplication;
    use application::{load_in, Format};
    use assets_test::{ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};
    use game_model::config::AssetRecord;
    use object_model::config::CharacterDefinition;

    use super::ObjectLoader;

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_object_assets", false)
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
