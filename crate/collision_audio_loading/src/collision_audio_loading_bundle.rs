use std::path::PathBuf;

use amethyst::{
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
