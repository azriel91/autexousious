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

#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use state_registry::StateId;
    use stdio_input::StdioInputBundle;
    use stdio_spi::VariantAndTokens;

    use super::StdioCommandStdioBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(StdioInputBundle::new())
            .with_bundle(StdioCommandStdioBundle::new())
            .with_resource(StateId::Loading)
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<VariantAndTokens>>();
            })
            // kcov-ignore-end
            .run()
    }
}
