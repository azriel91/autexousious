use std::any;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::MapSelectionSystem;

/// Adds the `MapSelectionSystem` to the `World`.
#[derive(Debug, new)]
pub struct MapSelectionBundle {
    /// System names that the `MapSelectionSystem` should depend on.
    #[new(default)]
    system_dependencies: Option<Vec<&'static str>>,
}

impl MapSelectionBundle {
    /// Specifies system dependencies for the `MapSelectionSystem`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: Vec<&'static str>) -> Self {
        self.system_dependencies = Some(dependencies);
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for MapSelectionBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self.system_dependencies.unwrap_or_else(Vec::new);
        builder.add(
            MapSelectionSystem::new(),
            any::type_name::<MapSelectionSystem>(),
            &deps,
        ); // kcov-ignore

        Ok(())
    }
}

// TODO: Custom state dispatcher with bundles.
// See <https://gitlab.com/azriel91/autexousious/issues/74>.
