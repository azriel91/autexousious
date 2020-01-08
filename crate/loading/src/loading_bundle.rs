use std::{any, path::PathBuf};

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;

use crate::{
    AssetDefinitionLoadingSystem, AssetDiscoverySystem, AssetIdMappingSystem,
    AssetPartLoadingCoordinatorSystem, AssetSequenceComponentLoadingSystem,
    AssetSpritesDefinitionLoadingSystem, AssetTextureLoadingSystem,
};

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
            AssetDiscoverySystem::new(self.assets_dir),
            any::type_name::<AssetDiscoverySystem>(),
            &[],
        ); // kcov-ignore
        builder.add(
            AssetPartLoadingCoordinatorSystem::new(),
            any::type_name::<AssetPartLoadingCoordinatorSystem>(),
            &[any::type_name::<AssetDiscoverySystem>()],
        ); // kcov-ignore
        builder.add(
            AssetDefinitionLoadingSystem::new(),
            any::type_name::<AssetDefinitionLoadingSystem>(),
            &[any::type_name::<AssetPartLoadingCoordinatorSystem>()],
        ); // kcov-ignore
        builder.add(
            AssetIdMappingSystem::new(),
            any::type_name::<AssetIdMappingSystem>(),
            &[any::type_name::<AssetDefinitionLoadingSystem>()],
        ); // kcov-ignore
        builder.add(
            AssetSpritesDefinitionLoadingSystem::new(),
            any::type_name::<AssetSpritesDefinitionLoadingSystem>(),
            &[any::type_name::<AssetIdMappingSystem>()],
        ); // kcov-ignore
        builder.add(
            AssetTextureLoadingSystem::new(),
            any::type_name::<AssetTextureLoadingSystem>(),
            &[any::type_name::<AssetSpritesDefinitionLoadingSystem>()],
        ); // kcov-ignore
        builder.add(
            AssetSequenceComponentLoadingSystem::new(),
            any::type_name::<AssetSequenceComponentLoadingSystem>(),
            &[any::type_name::<AssetTextureLoadingSystem>()],
        ); // kcov-ignore
        Ok(())
    }
}
