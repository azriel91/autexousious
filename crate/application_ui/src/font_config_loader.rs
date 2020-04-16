use amethyst::Error;
use application::{AppDir, AppFile, Format};

use crate::FontConfig;

/// Provides functionality to load `FontConfig`.
#[derive(Debug)]
pub struct FontConfigLoader;

impl FontConfigLoader {
    /// Loads `FontConfig` from the default path.
    pub fn load() -> Result<FontConfig, Error> {
        Self::load_path(AppDir::RESOURCES, "font_config.ron")
    }

    /// Loads `FontConfig` from configuration bytes.
    pub fn load_bytes(theme_bytes: &[u8]) -> Result<FontConfig, Error> {
        AppFile::load_bytes::<FontConfig>(theme_bytes, Format::Ron)
    }

    /// Loads `FontConfig` from the specified path.
    pub fn load_path(conf_dir: &str, font_config_name: &str) -> Result<FontConfig, Error> {
        AppFile::load_in::<FontConfig, _>(conf_dir, font_config_name, Format::Ron)
    }
}
