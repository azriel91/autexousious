use amethyst::{prelude::*, renderer::MaterialTextureSet};
use game_model::config::ConfigRecord;
use object_model::{
    config::{object::SequenceId, ObjectDefinition},
    loaded,
};

use animation::MaterialAnimationLoader;
use error::Result;
use sprite::SpriteLoader;

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

        let (sprite_sheets, mesh, mesh_mirrored, default_material) =
            SpriteLoader::load(world, texture_index_offset, config_record)?;

        let animation_handles = MaterialAnimationLoader::load(
            world,
            object_definition,
            texture_index_offset,
            &sprite_sheets,
        )?;

        Ok(loaded::Object::new(
            default_material,
            mesh,
            mesh_mirrored,
            animation_handles,
        ))
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use amethyst_test_support::AmethystApplication;
    use application::resource::dir::assets_dir;
    use game_model::config::ConfigRecord;
    use object_model::config::CharacterDefinition;
    use toml;

    use super::ObjectLoader;
    use IoUtils;

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

                    let object_toml = IoUtils::read_file(
                        &config_record.directory.join("object.toml"),
                    ).expect("Failed to read object.toml");
                    let character_definition =
                        toml::from_slice::<CharacterDefinition>(&object_toml)
                            .expect("Failed to parse object.toml into CharacterDefinition");

                    let object = ObjectLoader::load(
                        world,
                        &config_record,
                        &character_definition.object_definition,
                    ).expect("Failed to load object");

                    // See bat/object.toml
                    assert_eq!(8, object.animations.len());
                })
                .run()
                .is_ok()
        );
    }
}
