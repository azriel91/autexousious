#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use character_model::{config::CharacterDefinition, loaded::CharacterIrs};

    use character_loading::CharacterLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CharacterLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<CharacterDefinition>>();
                world.read_resource::<AssetStorage<CharacterIrs>>();
            })
            .run()
    }
}
