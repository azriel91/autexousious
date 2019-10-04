use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use application_event::AppEventVariant;
use derive_new::new;
use stdio_input::StdinSystem;
use stdio_spi::MapperSystem;
use typename::TypeName;

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
            &MapperSystem::<StdioCommandEventStdinMapper>::type_name(),
            &[&StdinSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            StdioCommandProcessingSystem::new(),
            &StdioCommandProcessingSystem::type_name(),
            &[&MapperSystem::<StdioCommandEventStdinMapper>::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}
