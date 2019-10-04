#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use energy_model::config::EnergyDefinition;

    use energy_loading::EnergyLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(EnergyLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<EnergyDefinition>>();
            })
            .run()
    }
}
