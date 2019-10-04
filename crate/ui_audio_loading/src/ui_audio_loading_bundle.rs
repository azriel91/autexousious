use std::path::PathBuf;

use amethyst::{
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
