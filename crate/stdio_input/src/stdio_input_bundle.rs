use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::StdinSystem;

/// Adds the `StdinSystem` to the `World`.
#[derive(Debug, new)]
pub struct StdioInputBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioInputBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(StdinSystem::new(), &StdinSystem::type_name(), &[]);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{ecs::WorldExt, shrev::EventChannel, Error};
    use amethyst_test::AmethystApplication;
    use application_input::ApplicationEvent;
    use state_registry::StateId;

    use super::StdioInputBundle;

    #[test]
    fn bundle_should_add_stdin_system_to_dispatcher() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(StdioInputBundle)
            .with_resource(StateId::Loading)
            // kcov-ignore-start
            .with_effect(|world| {
                world.read_resource::<EventChannel<ApplicationEvent>>();
            })
            // kcov-ignore-end
            .run()
    }
}
