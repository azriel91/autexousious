use std::path::PathBuf;

use amethyst::{
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;

use crate::{AssetDiscoverySystem, AssetLoadingSystem};

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
            AssetLoadingSystem::new(),
            &AssetLoadingSystem::type_name(),
            &[&AssetDiscoverySystem::type_name()],
        ); // kcov-ignore
        Ok(())
    }
}
