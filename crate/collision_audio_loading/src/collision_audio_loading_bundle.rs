use std::path::PathBuf;

use amethyst::ecs::WorldExt; use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use collision_audio_model::config::CollisionSfxPaths;
use derive_new::new;
use typename::TypeName;

use crate::CollisionAudioLoadingSystem;

/// Adds the following systems to the `World`:
///
/// * `Processor<CollisionSfxPaths>`
/// * `CollisionAudioLoadingSystem`
#[derive(Debug, new)]
pub struct CollisionAudioLoadingBundle {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for CollisionAudioLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<CollisionSfxPaths>::new(),
            "collision_sfx_paths_processor",
            &[],
        ); // kcov-ignore
        builder.add(
            CollisionAudioLoadingSystem::new(self.assets_dir),
            &CollisionAudioLoadingSystem::type_name(),
            &["collision_sfx_paths_processor"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use amethyst::ecs::WorldExt; use amethyst::{assets::AssetStorage, Error};
    use amethyst_test::AmethystApplication;
    use collision_audio_model::{
        config::CollisionSfxPaths, loaded::CollisionSfxMap, CollisionAudioLoadingStatus,
    };

    use super::CollisionAudioLoadingBundle;

    #[test]
    fn bundle_build_adds_collision_resources() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(CollisionAudioLoadingBundle::new(PathBuf::default()))
            .with_assertion(|world| {
                // Panics if the Systems weren't added
                world.read_resource::<AssetStorage<CollisionSfxPaths>>();

                world.read_resource::<CollisionAudioLoadingStatus>();
                world.read_resource::<CollisionSfxMap>();
            })
            .run()
    }
}
