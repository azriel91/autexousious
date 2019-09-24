use std::path::PathBuf;

use asset_model::config::{AssetSlug, AssetSlugBuilder, AssetTypeVariants};
use lazy_static::lazy_static;

use crate::{NAMESPACE_TEST, NAMESPACE_TEST_PATH};

/// Name of the "fade" map asset.
pub const MAP_FADE_NAME: &str = "fade";

/// Name of the "empty" map asset.
pub const MAP_EMPTY_NAME: &str = "empty";

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    static ref MAP_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(AssetTypeVariants::Map.to_string());

    /// Slug of the "fade" map asset.
    pub static ref MAP_FADE_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(MAP_FADE_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                MAP_FADE_NAME,
                e
            ))
    };

    /// `PathBuf` to the "fade" map asset directory.
    pub static ref MAP_FADE_PATH: PathBuf = MAP_PATH.join(MAP_FADE_NAME);

    /// Slug of the "empty" map asset.
    pub static ref MAP_EMPTY_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(MAP_EMPTY_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                MAP_EMPTY_NAME,
                e
            ))
    };

    /// `PathBuf` to the "fade" map asset directory.
    pub static ref MAP_EMPTY_PATH: PathBuf = MAP_PATH.join(MAP_EMPTY_NAME);
}
