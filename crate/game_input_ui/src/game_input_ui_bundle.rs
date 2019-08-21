use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use game_input_model::InputConfig;
use typename::TypeName;

use crate::InputToControlInputSystem;

/// Adds the game input update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameInputUiBundle {
    /// All controller input configuration.
    input_config: InputConfig,
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputUiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            InputToControlInputSystem::new(self.input_config),
            &InputToControlInputSystem::type_name(),
            &["input_system"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst_test::prelude::*;
    use game_input_model::{ControlBindings, InputConfig};

    use super::GameInputUiBundle;

    #[test]
    fn bundle_build_should_succeed() {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        // kcov-ignore-start
        assert!(
            // kcov-ignore-end
            AmethystApplication::ui_base::<ControlBindings>()
                .with_bundle(GameInputUiBundle::new(InputConfig::default()))
                .run()
                .is_ok()
        );
    }
}
