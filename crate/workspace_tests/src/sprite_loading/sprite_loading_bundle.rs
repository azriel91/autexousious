#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use sprite_model::{
        config::SpritesDefinition,
        loaded::{ScaleSequence, SpriteRenderSequence, TintSequence},
    };

    use sprite_loading::SpriteLoadingBundle;

    #[test]
    fn bundle_build_adds_sprite_processor() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(SpriteLoadingBundle)
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<SpritesDefinition>>();
                world.read_resource::<AssetStorage<SpriteRenderSequence>>();
                world.read_resource::<AssetStorage<TintSequence>>();
                world.read_resource::<AssetStorage<ScaleSequence>>();
            })
            .run()
    }
}
