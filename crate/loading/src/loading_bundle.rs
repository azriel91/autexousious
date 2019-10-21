use std::path::PathBuf;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

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
            AssetDiscoverySystem::new(self.assets_dir.clone()),
            &AssetDiscoverySystem::type_name(),
            &[],
        ); // kcov-ignore
        builder.add(
            AssetPartLoadingCoordinatorSystem::new(),
            &AssetPartLoadingCoordinatorSystem::type_name(),
            &[&AssetDiscoverySystem::type_name()],
        ); // kcov-ignore
        builder.add(
            AssetDefinitionLoadingSystem::new(),
            &AssetDefinitionLoadingSystem::type_name(),
            &[&AssetPartLoadingCoordinatorSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            AssetIdMappingSystem::new(),
            &AssetIdMappingSystem::type_name(),
            &[&AssetDefinitionLoadingSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            AssetSpritesDefinitionLoadingSystem::new(),
            &AssetSpritesDefinitionLoadingSystem::type_name(),
            &[&AssetIdMappingSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            AssetTextureLoadingSystem::new(),
            &AssetTextureLoadingSystem::type_name(),
            &[&AssetSpritesDefinitionLoadingSystem::type_name()],
        ); // kcov-ignore
        builder.add(
            AssetSequenceComponentLoadingSystem::new(),
            &AssetSequenceComponentLoadingSystem::type_name(),
            &[&AssetTextureLoadingSystem::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}
