use std::path::PathBuf;

use game_model::config::{AssetSlug, AssetSlugBuilder};
use heck::SnakeCase;
use object_model::ObjectType;

use ASSETS_OBJECT_PATH;
use NAMESPACE_TEST;

/// Name of the "bat" character asset.
pub const ASSETS_CHAR_BAT_NAME: &str = "bat";
/// File name of the grey bat sprites.
pub const ASSETS_CHAR_BAT_SPRITE_GREY_NAME: &str = "bat_grey.png";
/// File name of the brown bat sprites.
pub const ASSETS_CHAR_BAT_SPRITE_BROWN_NAME: &str = "bat_brown.png";

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    static ref ASSETS_CHAR_PATH: PathBuf =
        ASSETS_OBJECT_PATH.join(ObjectType::Character.to_string().to_snake_case());

    /// Slug of the "bat" character asset.
    pub static ref ASSETS_CHAR_BAT_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(ASSETS_CHAR_BAT_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                ASSETS_CHAR_BAT_NAME,
                e
            ))
    };

    /// `PathBuf` to the "bat" character asset directory.
    pub static ref ASSETS_CHAR_BAT_PATH: PathBuf = ASSETS_CHAR_PATH.join(ASSETS_CHAR_BAT_NAME);
}
