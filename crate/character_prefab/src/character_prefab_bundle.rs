use amethyst::{
    assets::PrefabLoaderSystemDesc,
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::CharacterPrefab;

/// Name of the `PrefabLoaderSystem<CharacterPrefab>`.
pub const CHARACTER_PREFAB_LOADER_SYSTEM: &str = "character_prefab_loader_system";

/// Adds the following `System`s to the world:
///
/// * `PrefabLoaderSystem::<CharacterPrefab>`
#[derive(Debug, new)]
pub struct CharacterPrefabBundle {
    /// System names that the `PrefabLoaderSystem::<CharacterPrefab>` should depend on.
    #[new(default)]
    system_dependencies: Option<Vec<String>>,
}

impl CharacterPrefabBundle {
    /// Specifies system dependencies for the `PrefabLoaderSystem::<CharacterPrefab>`.
    ///
    /// # Parameters
    ///
    /// * `dependencies`: Names of the systems to depend on.
    pub fn with_system_dependencies(mut self, dependencies: &[String]) -> Self {
        self.system_dependencies = Some(Vec::from(dependencies));
        self
    }
}

impl<'a, 'b> SystemBundle<'a, 'b> for CharacterPrefabBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let deps = self
            .system_dependencies
            .as_ref()
            .map_or_else(Vec::new, |deps| {
                deps.iter().map(AsRef::as_ref).collect::<Vec<&str>>()
            });

        builder.add(
            PrefabLoaderSystemDesc::<CharacterPrefab>::default().build(world),
            CHARACTER_PREFAB_LOADER_SYSTEM,
            &deps,
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use amethyst::{
        assets::{AssetStorage, Prefab},
        ecs::WorldExt,
        Error,
    };
    use amethyst_test::AmethystApplication;

    use super::CharacterPrefabBundle;
    use crate::CharacterPrefab;

    #[test]
    fn bundle_build() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CharacterPrefabBundle::new())
            .with_assertion(|world| {
                // Panics if the Systems are not added.
                world.read_resource::<AssetStorage<Prefab<CharacterPrefab>>>();
            })
            .run_isolated()
    }
}
