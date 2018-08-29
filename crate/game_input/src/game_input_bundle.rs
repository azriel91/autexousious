use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use ControllerInputUpdateSystem;
use InputConfig;

/// Adds the game input update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameInputBundle {
    /// All controller input configuration.
    input_config: InputConfig,
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            ControllerInputUpdateSystem::new(self.input_config),
            &ControllerInputUpdateSystem::type_name(),
            &["input_system"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test_support::prelude::*;

    use super::GameInputBundle;
    use InputConfig;
    use PlayerActionControl;
    use PlayerAxisControl;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<PlayerAxisControl, PlayerActionControl>()
                .with_bundle(GameInputBundle::new(InputConfig::default()))
                .run()
                .is_ok()
        );
    }
}
