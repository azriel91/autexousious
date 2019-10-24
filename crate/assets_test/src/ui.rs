use std::path::PathBuf;

use asset_model::config::{AssetSlug, AssetSlugBuilder, AssetTypeVariant};
use lazy_static::lazy_static;

use crate::{NAMESPACE_TEST, NAMESPACE_TEST_PATH};

/// Name of the "plain_background" map asset.
pub const UI_PLAIN_BACKGROUND_NAME: &str = "plain_background";

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    static ref UI_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(AssetTypeVariant::Ui.to_string());

    /// Slug of the "plain_background" map asset.
    pub static ref UI_PLAIN_BACKGROUND_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(UI_PLAIN_BACKGROUND_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                UI_PLAIN_BACKGROUND_NAME,
                e
            ))
    };

    /// `PathBuf` to the "fade" map asset directory.
    pub static ref UI_PLAIN_BACKGROUND_PATH: PathBuf = UI_PATH.join(UI_PLAIN_BACKGROUND_NAME);
}
