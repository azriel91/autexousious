use std::hash::Hash;

use amethyst::prelude::*;
use game_model::config::ConfigRecord;
use object_model::{config::ObjectDefinition, loaded};

use animation::MaterialAnimationLoader;
use error::Result;
use sprite::SpriteLoader;

/// Loads assets specified by object configuration into the loaded object model.
#[derive(Debug, Default, new)]
pub struct ObjectLoader {
    /// Offset for texture indices in the `MaterialTextureSet`
    #[new(default)]
    texture_index_offset: usize,
}

impl ObjectLoader {
    /// Returns the loaded `Object` referenced by the configuration record.
    ///
    /// # Parameters
    ///
    /// * `world`: `World` to store the object's assets.
    /// * `config_record`: Entry of the object's configuration.
    /// * `object_definition`: Object definition configuration.
    pub fn load<SeqId: Hash + Eq>(
        &mut self,
        world: &World,
        config_record: &ConfigRecord,
        object_definition: &ObjectDefinition<SeqId>,
    ) -> Result<loaded::Object> {
        let texture_index_offset = self.texture_index_offset;

        let (sprite_sheets, mesh, default_material) =
            SpriteLoader::load(world, texture_index_offset, config_record)?;

        let animation_handles = MaterialAnimationLoader::load(
            world,
            object_definition,
            texture_index_offset,
            &sprite_sheets,
        )?;

        self.texture_index_offset += sprite_sheets.len();

        Ok(loaded::Object::new(
            default_material,
            mesh,
            animation_handles,
        ))
    }
}

#[cfg(test)]
mod test {
    use std::path::{Path, PathBuf};

    use amethyst;
    use amethyst::animation::AnimationBundle;
    use amethyst::core::transform::TransformBundle;
    use amethyst::input::InputBundle;
    use amethyst::prelude::*;
    use amethyst::renderer::{
        ColorMask, DisplayConfig, DrawFlat, Material, Pipeline, PosTex, RenderBundle, Stage, ALPHA,
    };
    use amethyst::ui::UiBundle;
    use application::resource::dir::{self, assets_dir};
    use application::resource::find_in;
    use game_model::config::ConfigRecord;
    use object_model::config::CharacterDefinition;
    use toml;

    use super::ObjectLoader;
    use IoUtils;

    #[test]
    fn loads_object_assets() {
        assert!(run("loads_object_assets".to_string()).is_ok());
    }

    fn run(test_name: String) -> Result<(), amethyst::Error> {
        let assets_dir = assets_dir(Some(development_base_dirs!()))?;
        let mut display_config = DisplayConfig::load(
            find_in(
                dir::RESOURCES,
                "display_config.ron",
                Some(development_base_dirs!()),
            ).unwrap(),
        );
        display_config.title = test_name;

        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.1, 0.1, 0.1, 1.], 1.)
                .with_pass(DrawFlat::<PosTex>::new().with_transparency(
                    ColorMask::all(),
                    ALPHA,
                    None,
                )),
        );

        let mut app = Application::build(assets_dir.clone(), TestState { assets_dir })?
            // Needed to register `MaterialTextureSet`
            .with_bundle(AnimationBundle::<u32, Material>::new(
                "animation_control_system",
                "sampler_interpolation_system",
            ))?
            .with_bundle(
                TransformBundle::new()
                    .with_dep(&["animation_control_system", "sampler_interpolation_system"]),
            )?
            .with_bundle(InputBundle::<String, String>::new())?
            .with_bundle(UiBundle::<String, String>::new())?
            .with_bundle(RenderBundle::new(pipe, Some(display_config)))?
            .build()
            .expect("Failed to build application.");

        app.run();

        Ok(())
    }

    #[derive(Debug)]
    struct TestState {
        assets_dir: PathBuf,
    }
    impl<'a, 'b> State<GameData<'a, 'b>> for TestState {
        fn on_start(&mut self, world: &mut World) {
            let mut bat_path = self.assets_dir.clone();
            bat_path.extend(Path::new("test/object/character/bat").iter());
            let config_record = ConfigRecord::new(bat_path);

            let object_toml = IoUtils::read_file(&config_record.directory.join("object.toml"))
                .expect("Failed to read object.toml");
            let character_definition = toml::from_slice::<CharacterDefinition>(&object_toml)
                .expect("Failed to parse object.toml into CharacterDefinition");

            let object = ObjectLoader::new()
                .load(
                    world,
                    &config_record,
                    &character_definition.object_definition,
                )
                .expect("Failed to load object");

            assert_eq!(2, object.animations.len());
        }

        fn update(&mut self, _: &mut World) -> Trans {
            Trans::Quit
        }
    }
}
