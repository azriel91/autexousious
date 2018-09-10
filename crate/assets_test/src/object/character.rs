use std::path::PathBuf;

use game_model::config::{AssetSlug, AssetSlugBuilder};
use heck::SnakeCase;
use object_model::ObjectType;

use ASSETS_OBJECT_PATH;
use NAMESPACE_TEST;

/// Name of the "bat" character asset.
pub const ASSETS_CHAR_BAT_NAME: &str = "bat";

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
            .expect(&format!(
                "Expected `{}/{}` asset slug to build.",
                NAMESPACE_TEST,
                ASSETS_CHAR_BAT_NAME
            ))
    };

    /// `PathBuf` to the "bat" character asset directory.
    pub static ref ASSETS_CHAR_BAT_PATH: PathBuf = ASSETS_CHAR_PATH.join(ASSETS_CHAR_BAT_NAME);
}
