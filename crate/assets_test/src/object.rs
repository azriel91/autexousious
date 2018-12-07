use std::path::PathBuf;

use game_model::config::ConfigType;
use heck::SnakeCase;

use crate::NAMESPACE_TEST_PATH;

pub use self::character::{
    ASSETS_CHAR_BAT_NAME, ASSETS_CHAR_BAT_PATH, ASSETS_CHAR_BAT_SLUG,
    ASSETS_CHAR_BAT_SPRITE_BROWN_NAME, ASSETS_CHAR_BAT_SPRITE_GREY_NAME,
};

mod character;

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    pub static ref ASSETS_OBJECT_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(ConfigType::Object.to_string().to_snake_case());
}
