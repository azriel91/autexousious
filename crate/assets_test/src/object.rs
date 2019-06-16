use std::path::PathBuf;

use asset_model::config::ConfigType;
use lazy_static::lazy_static;

use crate::NAMESPACE_TEST_PATH;

pub use self::character::{
    CHAR_BAT_NAME, CHAR_BAT_PATH, CHAR_BAT_SLUG, CHAR_BAT_SPRITE_BROWN_NAME,
    CHAR_BAT_SPRITE_GREY_NAME,
};

mod character;

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    pub static ref OBJECT_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(ConfigType::Object.to_string());
}
