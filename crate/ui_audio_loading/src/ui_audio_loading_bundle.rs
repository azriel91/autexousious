use std::path::PathBuf;

use amethyst::ecs::WorldExt; use amethyst::{
    assets::Processor,
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World},
    Error,
};
use derive_new::new;
use typename::TypeName;
use ui_audio_model::config::UiSfxPaths;

use crate::UiAudioLoadingSystem;

/// Adds the following systems to the `World`:
///
/// * `Processor<UiSfxPaths>`
/// * `UiAudioLoadingSystem`
#[derive(Debug, new)]
pub struct UiAudioLoadingBundle {
    /// Path to the assets directory.
    assets_dir: PathBuf,
}

impl<'a, 'b> SystemBundle<'a, 'b> for UiAudioLoadingBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(
            Processor::<UiSfxPaths>::new(),
            "ui_sfx_paths_processor",
            &[],
        ); // kcov-ignore
        builder.add(
            UiAudioLoadingSystem::new(self.assets_dir),
            &UiAudioLoadingSystem::type_name(),
            &["ui_sfx_paths_processor"],
        ); // kcov-ignore
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use amethyst::ecs::WorldExt; use amethyst::{assets::AssetStorage, Error};
    use amethyst_test::AmethystApplication;
    use ui_audio_model::{config::UiSfxPaths, loaded::UiSfxMap, UiAudioLoadingStatus};

    use super::UiAudioLoadingBundle;

    #[test]
    fn bundle_build_adds_ui_resources() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(UiAudioLoadingBundle::new(PathBuf::default()))
            .with_assertion(|world| {
                // Panics if the Systems weren't added
                world.read_resource::<AssetStorage<UiSfxPaths>>();

                world.read_resource::<UiAudioLoadingStatus>();
                world.read_resource::<UiSfxMap>();
            })
            .run()
    }
}
