use amethyst::{
    assets::PrefabLoaderSystem, core::bundle::SystemBundle, ecs::DispatcherBuilder, Error,
};
use derive_new::new;

use crate::EnergyPrefab;

/// Name of the `PrefabLoaderSystem<EnergyPrefab>`.
pub const ENERGY_PREFAB_LOADER_SYSTEM: &str = "energy_prefab_loader_system";

/// Adds the following `System`s to the world:
///
/// * `PrefabLoaderSystem::<EnergyPrefab>`
#[derive(Debug, new)]
pub struct EnergyPrefabBundle {
    /// System names that the `PrefabLoaderSystem::<EnergyPrefab>` should depend on.
    #[new(default)]
    system_dependencies: Option<Vec<String>>,
}

impl EnergyPrefabBundle {
    /// Specifies system dependencies for the `PrefabLoaderSystem::<EnergyPrefab>`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: &[String]) -> Self {
        self.system_dependencies = Some(Vec::from(dependencies));
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for EnergyPrefabBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        let deps = self
            .system_dependencies
            .as_ref()
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
            });

        builder.add(
            PrefabLoaderSystem::<EnergyPrefab>::default(),
            ENERGY_PREFAB_LOADER_SYSTEM,
            &deps,
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Prefab},
        Error,
    };
    use amethyst_test::AmethystApplication;

    use super::EnergyPrefabBundle;
    use crate::EnergyPrefab;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(EnergyPrefabBundle::new())
            .with_assertion(|world| {
                // Panics if the Systems are not added.
                world.read_resource::<AssetStorage<Prefab<EnergyPrefab>>>();
            })
            .run_isolated()
    }
}
