use std::path::PathBuf;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    audio::{FlacFormat, Mp3Format, OggFormat, Source, WavFormat},
    ecs::{Read, ReadExpect, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::TomlFormat;
use derivative::Derivative;
use derive_new::new;
use log::{debug, error};
use typename_derive::TypeName;
use ui_audio_model::{
    config::{UiSfxId, UiSfxPaths},
    loaded::UiSfxMap,
    UiAudioLoadingStatus,
};

/// File name of the UI audio configuration.
const UI_AUDIO_TOML: &str = "ui_audio.toml";

/// Loads sound effect (SFX) assets.
#[derive(Default, Derivative, TypeName, new)]
#[derivative(Debug)]
pub struct UiAudioLoadingSystem {
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// `Handle` to the `UiSfxPaths`.
    #[new(default)]
    ui_sfx_paths_handle: Option<Handle<UiSfxPaths>>,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    progress_counter: ProgressCounter,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct UiAudioLoadingSystemData<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    loader: ReadExpect<'s, Loader>,
    /// `UiSfxPaths` assets.
    #[derivative(Debug = "ignore")]
    ui_sfx_paths_assets: Read<'s, AssetStorage<UiSfxPaths>>,
    /// `Source` assets.
    #[derivative(Debug = "ignore")]
    source_assets: Read<'s, AssetStorage<Source>>,
    /// `UiSfxMap` resource.
    #[derivative(Debug = "ignore")]
    ui_sfx_map: Write<'s, UiSfxMap>,
    /// `UiAudioLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    ui_audio_loading_status: Write<'s, UiAudioLoadingStatus>,
}

impl<'s> System<'s> for UiAudioLoadingSystem {
    type SystemData = UiAudioLoadingSystemData<'s>;

    fn run(
        &mut self,
        UiAudioLoadingSystemData {
            loader,
            ui_sfx_paths_assets,
            source_assets,
            mut ui_sfx_map,
            mut ui_audio_loading_status,
        }: Self::SystemData,
    ) {
        if *ui_audio_loading_status == UiAudioLoadingStatus::NotStarted {
            *ui_audio_loading_status = UiAudioLoadingStatus::InProgress;

            let ui_audio_toml_path = self.assets_dir.join(UI_AUDIO_TOML);

            if ui_audio_toml_path.exists() {
                // Borrow self piecewise.
                let progress_counter = &mut self.progress_counter;
                let ui_sfx_paths_handle = &mut self.ui_sfx_paths_handle;
                let handle = loader.load(
                    UI_AUDIO_TOML,
                    TomlFormat,
                    progress_counter,
                    &ui_sfx_paths_assets,
                );
                *ui_sfx_paths_handle = Some(handle);
            } else {
                error!(
                    "Expected `{}` to exist in `assets` directory.",
                    UI_AUDIO_TOML
                );
                *ui_audio_loading_status = UiAudioLoadingStatus::Complete;
            }
        }

        if *ui_audio_loading_status == UiAudioLoadingStatus::InProgress {
            if let Some(ui_sfx_paths_handle) = self.ui_sfx_paths_handle.as_ref() {
                // If UiSfxMap is not loaded, begin loading it.
                if let Some(ui_sfx_paths) = ui_sfx_paths_assets.get(ui_sfx_paths_handle) {
                    debug!("Ui sfx paths: {:?}", &*ui_sfx_paths);
                    // Find keys -- `UiSfxId`s -- that are not part of the map.
                    // Begin loading it, add the handles for it to the map.
                    // Wait for all of the handles to be loaded.

                    let sfx_to_load = ui_sfx_paths
                        .iter()
                        .filter(|(ui_sfx_id, _)| ui_sfx_map.get(ui_sfx_id).is_none())
                        .collect::<Vec<(&UiSfxId, &PathBuf)>>();

                    sfx_to_load.into_iter().for_each(|(ui_sfx_id, path)| {
                        macro_rules! load {
                            ($audio_format:expr) => {
                                loader.load(
                                    format!("{}", path.display()),
                                    $audio_format,
                                    &mut self.progress_counter,
                                    &source_assets,
                                )
                            };
                        }
                        let source_handle = match path.extension() {
                            Some(ext) => {
                                let ext = ext
                                    .to_str()
                                    .expect("Failed to convert extension to unicode string.")
                                    .to_lowercase();
                                match ext.as_ref() {
                                    "mp3" => load!(Mp3Format),
                                    "wav" => load!(WavFormat),
                                    "ogg" => load!(OggFormat),
                                    "flac" => load!(FlacFormat),
                                    ext => {
                                        error!(
                                            "Unsupported extension: \"{}\", \
                                             falling back to `wav`.",
                                            ext
                                        );
                                        load!(WavFormat)
                                    }
                                }
                            }
                            None => {
                                error!(
                                    "No extension for audio file \"{}\", \
                                     falling back to `wav`.",
                                    path.display()
                                );
                                load!(WavFormat)
                            }
                        };
                        ui_sfx_map.insert(*ui_sfx_id, source_handle);
                    });

                    let all_loaded = ui_sfx_map
                        .iter()
                        .all(|(_, source_handle)| source_assets.get(source_handle).is_some());
                    if all_loaded {
                        debug!("UI audio assets loaded: {:?}", &*ui_sfx_map);
                        *ui_audio_loading_status = UiAudioLoadingStatus::Complete;
                    }
                }
            }
        }
    }
}
