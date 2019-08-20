use amethyst::ecs::WorldExt; use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use energy_model::{config::EnergyDefinition, loaded::Energy};
use object_loading::ObjectDefinitionToWrapperProcessor;
use typename::TypeName;

/// Name of the `Processor<Energy>` system.
pub const ENERGY_PROCESSOR: &str = "energy_processor";

/// Adds the following processor `System`s to the world:
///
/// * `Processor::<EnergyDefinition>`
/// * `ObjectDefinitionToWrapperProcessor::<Energy>`
/// * `Processor::<Energy>`
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
        builder.add(
            ObjectDefinitionToWrapperProcessor::<Energy>::new(),
            &ObjectDefinitionToWrapperProcessor::<Energy>::type_name(),
            &["energy_definition_processor", "sprite_sheet_processor"],
        ); // kcov-ignore
        builder.add(
            Processor::<Energy>::new(),
            ENERGY_PROCESSOR,
            &[&ObjectDefinitionToWrapperProcessor::<Energy>::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::ecs::WorldExt; use amethyst::{
        assets::AssetStorage,
        core::TransformBundle,
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use energy_model::{
        config::EnergyDefinition,
        loaded::{Energy, EnergyObjectWrapper},
    };
    use sequence_loading::SequenceLoadingBundle;

    use super::EnergyLoadingBundle;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(EnergyLoadingBundle::new())
            .with_assertion(|world| {
                // Panics if the Processors are not added.
                world.read_resource::<AssetStorage<EnergyDefinition>>();
                world.read_resource::<AssetStorage<EnergyObjectWrapper>>();
                world.read_resource::<AssetStorage<Energy>>();
            })
            .run_isolated()
    }
}
