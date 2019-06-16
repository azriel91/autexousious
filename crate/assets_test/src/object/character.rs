use std::path::PathBuf;

use asset_model::config::{AssetSlug, AssetSlugBuilder};
use lazy_static::lazy_static;
use object_type::ObjectType;

use crate::{NAMESPACE_TEST, OBJECT_PATH};

/// Name of the "bat" character asset.
pub const CHAR_BAT_NAME: &str = "bat";
/// File name of the grey bat sprites.
pub const CHAR_BAT_SPRITE_GREY_NAME: &str = "bat_grey.png";
/// File name of the brown bat sprites.
pub const CHAR_BAT_SPRITE_BROWN_NAME: &str = "bat_brown.png";

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    static ref CHAR_PATH: PathBuf =
        OBJECT_PATH.join(ObjectType::Character.to_string());

    /// Slug of the "bat" character asset.
    pub static ref CHAR_BAT_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(CHAR_BAT_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                CHAR_BAT_NAME,
                e
            ))
    };

    /// `PathBuf` to the "bat" character asset directory.
    pub static ref CHAR_BAT_PATH: PathBuf = CHAR_PATH.join(CHAR_BAT_NAME);
}
