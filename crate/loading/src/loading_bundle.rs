use std::path::PathBuf;

use amethyst::{core::bundle::SystemBundle, ecs::DispatcherBuilder, Error};
use character_model::loaded::Character;
use character_prefab::CharacterPrefab;
use derive_new::new;
use typename::TypeName;

use crate::{MapAssetLoadingSystem, ObjectAssetLoadingSystem};

/// Adds the `ObjectAssetLoadingSystem<O, Pf>`s to the `World`.
#[derive(Debug, new)]
pub struct LoadingBundle {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for LoadingBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<(), Error> {
        builder.add(
            ObjectAssetLoadingSystem::<Character, CharacterPrefab>::new(self.assets_dir.clone()),
            &ObjectAssetLoadingSystem::<Character, CharacterPrefab>::type_name(),
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

    use amethyst::Error;
    use amethyst_test::AmethystApplication;
    use assets_test::ASSETS_PATH;
    use character_prefab::CharacterPrefab;
    use game_model::loaded::{GameObjectPrefabs, MapAssets};

    use super::LoadingBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(|world| {
                world.read_resource::<GameObjectPrefabs<CharacterPrefab>>();
                world.read_resource::<MapAssets>();
            })
            .run()
    }
}
