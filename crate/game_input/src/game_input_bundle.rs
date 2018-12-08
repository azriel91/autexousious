use amethyst::{
    core::bundle::{Result, SystemBundle},
    ecs::prelude::*,
};
use typename::TypeName;

use crate::{ControllerInputUpdateSystem, InputConfig, SharedControllerInputUpdateSystem};

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
        builder.add(
            SharedControllerInputUpdateSystem::new(),
            &SharedControllerInputUpdateSystem::type_name(),
            &[&ControllerInputUpdateSystem::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;

    use super::GameInputBundle;
    use crate::{InputConfig, PlayerActionControl, PlayerAxisControl};

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
