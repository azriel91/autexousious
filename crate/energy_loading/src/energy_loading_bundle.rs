use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use energy_model::config::EnergyDefinition;

/// Adds the following processor `System`s to the world:
///
/// * `Processor::<EnergyDefinition>`
#[derive(Debug, new)]
pub struct EnergyLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for EnergyLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<EnergyDefinition>::new(),
            "energy_definition_processor",
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{assets::AssetStorage, ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use energy_model::config::EnergyDefinition;

    use super::EnergyLoadingBundle;

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
