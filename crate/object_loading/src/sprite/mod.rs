pub(self) use self::material_creator::MaterialCreator;
pub(crate) use self::sprite_loader::SpriteLoader;
pub(self) use self::sprite_mesh_creator::SpriteMeshCreator;
pub(self) use self::sprite_sheet_mapper::SpriteSheetMapper;
pub(self) use self::texture_loader::TextureLoader;

mod material_creator;
mod sprite_loader;
mod sprite_mesh_creator;
mod sprite_sheet_mapper;
mod texture_loader;

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

    use super::SpriteLoader;

    // kcov-ignore-start
    // TODO: We still cannot run multiple windows in the same binary: #30
    #[test]
    #[ignore]
    fn loads_sprite_sheets_textures_and_mesh() {
        assert!(run("loads_sprite_sheets_textures_and_mesh".to_string()).is_ok());
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
            let texture_index_offset = 0;
            let mut bat_path = self.assets_dir.clone();
            bat_path.extend(Path::new("test/object/character/bat").iter());
            let config_record = ConfigRecord::new(bat_path);
            let result = SpriteLoader::load(world, texture_index_offset, &config_record);

            if let Err(e) = result {
                panic!("Failed to load sprites: {:?}", e); // kcov-ignore
            } // kcov-ignore
        }

        fn update(&mut self, _: &mut World) -> Trans {
            Trans::Quit
        }
    }
    // kcov-ignore-end
}
