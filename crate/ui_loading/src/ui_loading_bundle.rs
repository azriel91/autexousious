use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use ui_model::config::UiDefinition;

/// Adds the following `System`s to the `World`:
///
/// * `Processor<UiDefinition>`
#[derive(Debug, new)]
pub struct UiLoadingBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for UiLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<UiDefinition>::new(),
            "ui_definition_processor",
            &[],
        );
        Ok(())
    }
}
