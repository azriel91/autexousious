use amethyst::{prelude::*, renderer::SpriteSheetSet};
use application::Result;
use game_model::config::ConfigRecord;
use object_model::{
    config::{object::SequenceId, ObjectDefinition},
    loaded,
};
use sprite_loading::{SpriteLoader, SpriteRenderAnimationLoader};

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug)]
pub struct ObjectLoader;

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the configuration record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the object's assets.
    /// * `config_record`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<SeqId: SequenceId>(
        world: &World,
        config_record: &ConfigRecord,
        object_definition: &ObjectDefinition<SeqId>,
    ) -> Result<loaded::Object<SeqId>> {
        let sprite_sheet_index_offset = world.read_resource::<SpriteSheetSet>().len() as u64;

        debug!(
            "Loading object assets in `{}`",
            config_record.directory.display()
        );

        let (sprite_sheet_handles, _texture_handles) =
            SpriteLoader::load(world, sprite_sheet_index_offset, &config_record.directory)?;
        let sprite_sheet_handle = sprite_sheet_handles
            .into_iter()
            .next()
            .expect("Expected character to have at least one sprite sheet.");

        let animation_handles = SpriteRenderAnimationLoader::load_into_map(
            world,
            &object_definition.sequences,
            sprite_sheet_index_offset,
        );

        Ok(loaded::Object::new(sprite_sheet_handle, animation_handles))
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst_test_support::AmethystApplication;
    use application::{load_in, resource::dir::assets_dir, Format};
    use game_model::config::{AssetRefBuilder, ConfigRecord};
    use object_model::config::CharacterDefinition;

    use super::ObjectLoader;

    #[test]
    fn loads_object_assets() {
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::render_base("loads_object_assets", false)
                .with_assertion(|world| {
                    let mut bat_path = assets_dir(Some(development_base_dirs!())).unwrap();
                    bat_path.extend(Path::new("test/object/character/bat").iter());
                    let asset_ref = AssetRefBuilder::default()
                        .namespace("test".to_string())
                        .name("bat".to_string())
                        .build()
                        .expect("Failed to build `test/bat` asset ref.");
                    let config_record = ConfigRecord::new(asset_ref, bat_path);

                    let character_definition = load_in::<CharacterDefinition, _>(
                        &config_record.directory,
                        "object.toml",
                        Format::Toml,
                        None,
                    ).expect("Failed to load object.toml into CharacterDefinition");

                    let object = ObjectLoader::load(
                        world,
                        &config_record,
                        &character_definition.object_definition,
                    ).expect("Failed to load object");

                    // See bat/object.toml
                    assert_eq!(9, object.animations.len());
                }).run()
                .is_ok()
        );
    }
}
