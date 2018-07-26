use amethyst::{prelude::*, renderer::MaterialTextureSet};
use application::Result;
use game_model::config::ConfigRecord;
use object_model::{
    config::{object::SequenceId, ObjectDefinition},
    loaded,
};
use sprite_loading::{MaterialAnimationLoader, SpriteLoader};

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
        let texture_index_offset = world.read_resource::<MaterialTextureSet>().len() as u64;

        debug!(
            "Loading object assets in `{}`",
            config_record.directory.display()
        );

        let (sprite_sheets, sprite_material_mesh) =
            SpriteLoader::load(world, texture_index_offset, &config_record.directory)?;

        let animation_handles = MaterialAnimationLoader::load_into_map(
            world,
            &object_definition.sequences,
            texture_index_offset,
            &sprite_sheets,
        );

        Ok(loaded::Object::new(sprite_material_mesh, animation_handles))
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst_test_support::AmethystApplication;
    use application::{load_in, resource::dir::assets_dir, Format};
    use game_model::config::ConfigRecord;
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
                    let config_record = ConfigRecord::new(bat_path);

                    let character_definition =
                        load_in::<CharacterDefinition, _>(
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
                })
                .run()
                .is_ok()
        );
    }
}
