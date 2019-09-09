use std::path::Path;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Progress},
    audio::{FlacFormat, Mp3Format, OggFormat, Source, WavFormat},
};
use log::error;

/// Loads audio `Source` files.
#[derive(Debug)]
pub struct AudioLoader;

impl AudioLoader {
    /// Returns a `Handle` to the audio `Source`.
    pub fn load<P>(
        loader: &Loader,
        source_assets: &AssetStorage<Source>,
        progress: P,
        path: &Path,
    ) -> Handle<Source>
    where
        P: Progress,
    {
        macro_rules! load {
            ($audio_format:expr) => {
                loader.load(
                    format!("{}", path.display()),
                    $audio_format,
                    progress,
                    source_assets,
                )
            };
        }

        match path.extension() {
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
                        error!("Unknown extension: \"{}\", falling back to `wav`.", ext);
                        load!(WavFormat)
                    }
                }
            }
            None => {
                error!(
                    "No extension for audio file \"{}\",  falling back to `wav`.",
                    path.display()
                );
                load!(WavFormat)
            }
        }
    }
}
