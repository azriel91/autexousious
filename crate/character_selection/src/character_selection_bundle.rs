use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::CharacterSelectionSystem;

/// Adds the `CharacterSelectionSystem` to the `World`.
///
/// The Amethyst `InputBundle` must be added before this bundle.
#[derive(Debug, new)]
pub struct CharacterSelectionBundle {
    /// System names that the `CharacterSelectionSystem` should depend on.
    #[new(default)]
    system_dependencies: Option<Vec<&'static str>>,
}

impl CharacterSelectionBundle {
    /// Specifies system dependencies for the `CharacterSelectionSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: Vec<&'static str>) -> Self {
        self.system_dependencies = Some(dependencies);
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterSelectionBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self.system_dependencies.unwrap_or_else(Vec::new);

        builder.add(
            CharacterSelectionSystem::new(),
            any::type_name::<CharacterSelectionSystem>(),
            &deps,
        ); // kcov-ignore

        Ok(())
    }
}
