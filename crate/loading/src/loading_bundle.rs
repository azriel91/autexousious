use std::path::PathBuf;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use character_loading::CharacterLoadingStatus;
use character_model::loaded::Character;
use character_prefab::CharacterPrefab;
use derive_new::new;
use energy_loading::EnergyLoadingStatus;
use energy_model::loaded::Energy;
use energy_prefab::EnergyPrefab;
use typename::TypeName;

use crate::{AssetDiscoverySystem, MapAssetLoadingSystem, ObjectAssetLoadingSystem};

/// Adds asset discovery and loading systems to the `World`.
#[derive(Debug, new)]
pub struct LoadingBundle {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for LoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            AssetDiscoverySystem::new(self.assets_dir.clone()),
            &AssetDiscoverySystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            ObjectAssetLoadingSystem::<Character, CharacterPrefab, CharacterLoadingStatus>::new(self.assets_dir.clone()),
            &ObjectAssetLoadingSystem::<Character, CharacterPrefab, CharacterLoadingStatus>::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            ObjectAssetLoadingSystem::<Energy, EnergyPrefab, EnergyLoadingStatus>::new(
                self.assets_dir.clone(),
            ),
            &ObjectAssetLoadingSystem::<Energy, EnergyPrefab, EnergyLoadingStatus>::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            MapAssetLoadingSystem::new(self.assets_dir),
            &MapAssetLoadingSystem::type_name(),
            &[],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use assets_test::ASSETS_PATH;
    use character_prefab::CharacterPrefab;
    use energy_prefab::EnergyPrefab;
    use game_model::loaded::{GameObjectPrefabs, MapPrefabs};

    use super::LoadingBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(|world| {
                world.read_resource::<GameObjectPrefabs<CharacterPrefab>>();
                world.read_resource::<GameObjectPrefabs<EnergyPrefab>>();
                world.read_resource::<MapPrefabs>();
            })
            .run()
    }
}
