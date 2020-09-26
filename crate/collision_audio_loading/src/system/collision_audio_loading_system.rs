use std::path::PathBuf;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    audio::{FlacFormat, Mp3Format, OggFormat, Source, WavFormat},
    ecs::{Read, ReadExpect, System, World, Write},
    shred::{ResourceId, SystemData},
};
use asset_loading::YamlFormat;
use collision_audio_model::{
    config::CollisionSfxPaths, loaded::CollisionSfxMap, CollisionAudioLoadingStatus,
};
use derivative::Derivative;
use derive_new::new;
use log::{debug, error};
#[cfg(target_arch = "wasm32")]
use wasm_support_fs::PathAccessExt;

/// File name of the collision audio configuration.
const COLLISION_AUDIO_YAML: &str = "collision_audio.yaml";

/// Loads sound effect (SFX) assets.
#[derive(Default, Derivative, new)]
#[derivative(Debug)]
pub struct CollisionAudioLoadingSystem {
    /// Path to the assets directory.
    assets_dir: PathBuf,
    /// `Handle` to the `CollisionSfxPaths`.
    #[new(default)]
    collision_sfx_paths_handle: Option<Handle<CollisionSfxPaths>>,
    /// Tracks loaded assets.
    #[derivative(Debug = "ignore")]
    #[new(default)]
    progress_counter: ProgressCounter,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CollisionAudioLoadingSystemData<'s> {
    /// `Loader` to load assets.
    #[derivative(Debug = "ignore")]
    loader: ReadExpect<'s, Loader>,
    /// `CollisionSfxPaths` assets.
    #[derivative(Debug = "ignore")]
    collision_sfx_paths_assets: Read<'s, AssetStorage<CollisionSfxPaths>>,
    /// `Source` assets.
    #[derivative(Debug = "ignore")]
    source_assets: Read<'s, AssetStorage<Source>>,
    /// `CollisionSfxMap` resource.
    #[derivative(Debug = "ignore")]
    collision_sfx_map: Write<'s, CollisionSfxMap>,
    /// `CollisionAudioLoadingStatus` resource.
    #[derivative(Debug = "ignore")]
    collision_audio_loading_status: Write<'s, CollisionAudioLoadingStatus>,
}

impl<'s> System<'s> for CollisionAudioLoadingSystem {
    type SystemData = CollisionAudioLoadingSystemData<'s>;

    fn run(
        &mut self,
        CollisionAudioLoadingSystemData {
            loader,
            collision_sfx_paths_assets,
            source_assets,
            mut collision_sfx_map,
            mut collision_audio_loading_status,
        }: Self::SystemData,
    ) {
        if *collision_audio_loading_status == CollisionAudioLoadingStatus::NotStarted {
            *collision_audio_loading_status = CollisionAudioLoadingStatus::InProgress;

            let collision_audio_yaml_path = self.assets_dir.join(COLLISION_AUDIO_YAML);
            #[cfg(not(target_arch = "wasm32"))]
            let collision_audio_yaml_path_exists = collision_audio_yaml_path.exists();
            #[cfg(target_arch = "wasm32")]
            let collision_audio_yaml_path_exists = collision_audio_yaml_path.exists_on_server();

            if collision_audio_yaml_path_exists {
                // Borrow self piecewise.
                let progress_counter = &mut self.progress_counter;
                let collision_sfx_paths_handle = &mut self.collision_sfx_paths_handle;
                let handle = loader.load(
                    COLLISION_AUDIO_YAML,
                    YamlFormat,
                    progress_counter,
                    &collision_sfx_paths_assets,
                );
                *collision_sfx_paths_handle = Some(handle);
            } else {
                error!(
                    "Expected `{}` to exist in `assets` directory.",
                    COLLISION_AUDIO_YAML
                );
                *collision_audio_loading_status = CollisionAudioLoadingStatus::Complete;
            }
        }

        if *collision_audio_loading_status == CollisionAudioLoadingStatus::InProgress {
            if let Some(collision_sfx_paths_handle) = self.collision_sfx_paths_handle.as_ref() {
                // If CollisionSfxMap is not loaded, begin loading it.
                if let Some(collision_sfx_paths) =
                    collision_sfx_paths_assets.get(collision_sfx_paths_handle)
                {
                    debug!("Collision sfx paths: {:?}", &*collision_sfx_paths);
                    // Find keys -- `CollisionSfxId`s -- that are not part of the map.
                    // Begin loading it, add the handles for it to the map.
                    // Wait for all of the handles to be loaded.

                    collision_sfx_paths
                        .iter()
                        .for_each(|(collision_sfx_id, path)| {
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
                            if collision_sfx_map.get(collision_sfx_id).is_none() {
                                let source_handle = match path.extension() {
                                    Some(ext) => {
                                        let ext = ext
                                            .to_str()
                                            .expect(
                                                "Failed to convert extension to unicode string.",
                                            )
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
                                collision_sfx_map.insert(*collision_sfx_id, source_handle);
                            }
                        });

                    let all_loaded = collision_sfx_map
                        .iter()
                        .all(|(_, source_handle)| source_assets.get(source_handle).is_some());
                    if all_loaded {
                        debug!("Collision audio assets loaded: {:?}", &*collision_sfx_map);
                        *collision_audio_loading_status = CollisionAudioLoadingStatus::Complete;
                    }
                }
            }
        }
    }
}
