use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use derive_new::new;
use typename::TypeName;

use crate::StdinSystem;

/// Adds the `StdinSystem` to the `World`.
#[derive(Debug, new)]
pub struct StdioViewBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for StdioViewBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(StdinSystem::new(), &StdinSystem::type_name(), &[]);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::shrev::EventChannel;
    use amethyst_test::prelude::*;
    use application_input::ApplicationEvent;

    use super::StdioViewBundle;

    #[test]
    fn bundle_should_add_stdin_system_to_dispatcher() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));
        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::blank()
                .with_bundle(StdioViewBundle)
                // kcov-ignore-start
                .with_effect(|world| {
                    world.read_resource::<EventChannel<ApplicationEvent>>();
                })
                // kcov-ignore-end
                .run()
                .is_ok()
        );
    }
}
