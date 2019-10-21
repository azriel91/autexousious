mod sprite_loading_bundle;
mod sprite_sheet_mapper;

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Loader, ProgressCounter},
        core::TransformBundle,
        ecs::WorldExt,
        renderer::{types::DefaultBackend, RenderEmptyBundle, SpriteSheet, Texture},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application::{AppFile, Format};
    use assets_test::CHAR_BAT_PATH;
    use sprite_model::config::SpritesDefinition;

    use sprite_loading::SpriteLoader;

    #[test]
    fn loads_textures_and_sprite_sheets() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_assertion(|world| {
                let sprites_definition = AppFile::load_in::<SpritesDefinition, _>(
                    &*CHAR_BAT_PATH,
                    "sprites.yaml",
                    Format::Yaml,
                )
                .expect("Failed to load sprites_definition.");

                let loader = world.read_resource::<Loader>();
                let texture_assets = world.read_resource::<AssetStorage<Texture>>();
                let sprite_sheet_assets = world.read_resource::<AssetStorage<SpriteSheet>>();

                let result = SpriteLoader::load(
                    &mut ProgressCounter::default(),
                    &loader,
                    &texture_assets,
                    &sprite_sheet_assets,
                    &sprites_definition,
                    &CHAR_BAT_PATH,
                );

                if let Err(e) = result {
                    panic!("Failed to load sprites: {:?}", e); // kcov-ignore
                } // kcov-ignore
            })
            .run()
    }
}
