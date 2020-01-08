use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_input::StdinSystem;
use stdio_spi::MapperSystem;

use crate::{StdioCommandEventStdinMapper, StdioCommandProcessingSystem};

/// Adds a `MapperSystem<StdioCommandEventStdinMapper>` to the `World`.
#[derive(Debug, new)]
pub struct StdioCommandStdioBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioCommandStdioBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            MapperSystem::<StdioCommandEventStdinMapper>::new(AppEventVariant::StdioCommand),
            any::type_name::<MapperSystem<StdioCommandEventStdinMapper>>(),
            &[any::type_name::<StdinSystem>()],
        ); // kcov-ignore
        builder.add(
            StdioCommandProcessingSystem::new(),
            any::type_name::<StdioCommandProcessingSystem>(),
            &[any::type_name::<MapperSystem<StdioCommandEventStdinMapper>>()],
        ); // kcov-ignore
        Ok(())
    }
}
