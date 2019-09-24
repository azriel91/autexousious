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

#[cfg(test)]
mod test {
    use std::env;

    use amethyst::{ecs::WorldExt, Error};
    use amethyst_test::AmethystApplication;
    use asset_model::loaded::AssetTypeMappings;
    use assets_test::ASSETS_PATH;

    use super::LoadingBundle;

    #[test]
    fn bundle_should_add_mapper_system_to_dispatcher() -> Result<(), Error> {
        env::set_var("APP_DIR", env!("CARGO_MANIFEST_DIR"));

        AmethystApplication::blank()
            .with_bundle(LoadingBundle::new(ASSETS_PATH.clone()))
            .with_effect(|world| {
                world.read_resource::<AssetTypeMappings>();
            })
            .run()
    }
}
