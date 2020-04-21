use application::{AppFile, Format};
use game_input_model::config::PlayerInputConfigs;

/// Trait to acquire a built in value, when there is no configuration.
pub trait BuiltIn {
    /// Returns a built in value for `Self`.
    fn built_in() -> Self;
}

impl BuiltIn for PlayerInputConfigs {
    fn built_in() -> Self {
        let player_input_configs = include_bytes!("../resources/player_input_configs.yaml");
        AppFile::load_bytes(player_input_configs, Format::Yaml)
            .expect("Failed to load built-in `PlayerInputConfigs`.")
    }
}
