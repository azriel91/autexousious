use std::path::PathBuf;

use game_model::config::ConfigType;
use heck::SnakeCase;

use NAMESPACE_TEST_PATH;

pub use self::character::{ASSETS_CHAR_BAT_NAME, ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG};

mod character;

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    pub static ref ASSETS_OBJECT_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(ConfigType::Object.to_string().to_snake_case());
}
