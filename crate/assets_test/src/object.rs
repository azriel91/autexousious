use std::path::PathBuf;

use asset_model::config::AssetTypeVariants;
use lazy_static::lazy_static;

use crate::NAMESPACE_TEST_PATH;

pub use self::{
    character::{
        CHAR_BAT_NAME, CHAR_BAT_PATH, CHAR_BAT_SLUG, CHAR_BAT_SPRITE_BROWN_NAME,
        CHAR_BAT_SPRITE_GREY_NAME,
    },
    energy::{
        ENERGY_SQUARE_NAME, ENERGY_SQUARE_PATH, ENERGY_SQUARE_SLUG, ENERGY_SQUARE_SPRITE_NAME,
    },
};

mod character;
mod energy;

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    pub static ref OBJECT_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(AssetTypeVariants::Object.to_string());
}
