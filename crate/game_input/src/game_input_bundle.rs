use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::{ControllerInputUpdateSystem, SharedControllerInputUpdateSystem};

/// Adds the game input update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameInputBundle {
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
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self
            .system_dependencies
            .as_ref()
            // kcov-ignore-start
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
            });
        // kcov-ignore-end
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

    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use game_input_model::ControlBindings;

    use super::GameInputBundle;

    #[test]
    fn bundle_build_should_succeed() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::ui_base::<ControlBindings>()
            .with_bundle(GameInputBundle::new())
            .run()
    }
}
