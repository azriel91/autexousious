use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use derive_new::new;
use game_input_model::InputConfig;
use typename::TypeName;

use crate::{
    ControllerInputUpdateSystem, InputToControlInputSystem, SharedControllerInputUpdateSystem,
};

/// Adds the game input update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameInputBundle {
    /// All controller input configuration.
    input_config: InputConfig,
    /// System names that the `ControllerInputUpdateSystem` should wait on.
    #[new(default)]
    system_dependencies: Option<Vec<String>>,
}

impl GameInputBundle {
    /// Specifies system dependencies for the `ControllerInputUpdateSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: &[String]) -> Self {
        self.system_dependencies = Some(Vec::from(dependencies));
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let input_to_control_input_system_name = InputToControlInputSystem::type_name();

        builder.add(
            InputToControlInputSystem::new(self.input_config),
            &input_to_control_input_system_name,
            &["input_system"],
        ); // kcov-ignore

        let mut deps = self
            .system_dependencies
            .as_ref()
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(|dep| dep.as_ref()).collect::<Vec<&str>>()
            });
        deps.push(input_to_control_input_system_name.as_ref());
        builder.add(
            ControllerInputUpdateSystem::new(),
            &ControllerInputUpdateSystem::type_name(),
            &deps,
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
    use game_input_model::{InputConfig, PlayerActionControl, PlayerAxisControl};

    use super::GameInputBundle;

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
