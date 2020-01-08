use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::{ControllerInputUpdateSystem, SharedControllerInputUpdateSystem};

/// Adds the game input update systems to the provided dispatcher.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct GameInputBundle {
    /// System names that the `ControllerInputUpdateSystem` should wait on.
    #[new(default)]
    system_dependencies: Option<Vec<&'static str>>,
}

impl GameInputBundle {
    /// Specifies system dependencies for the `ControllerInputUpdateSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: Vec<&'static str>) -> Self {
        self.system_dependencies = Some(dependencies);
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for GameInputBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self.system_dependencies.unwrap_or_else(Vec::new);
        builder.add(
            ControllerInputUpdateSystem::new(),
            any::type_name::<ControllerInputUpdateSystem>(),
            &deps,
        ); // kcov-ignore

        builder.add(
            SharedControllerInputUpdateSystem::new(),
            any::type_name::<SharedControllerInputUpdateSystem>(),
            &[any::type_name::<ControllerInputUpdateSystem>()],
        ); // kcov-ignore
        Ok(())
    }
}
