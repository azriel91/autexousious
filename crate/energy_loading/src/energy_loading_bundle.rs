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
