use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use background_model::config::BackgroundDefinition;
use derive_new::new;

/// Adds the following `System`s to the `World`:
///
/// * `Processor<BackgroundDefinition>`
#[derive(Debug, new)]
pub struct BackgroundLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for BackgroundLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<BackgroundDefinition>::new(),
            "background_definition_processor",
            &[],
        );
        Ok(())
    }
}
