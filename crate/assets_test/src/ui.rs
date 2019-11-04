use std::path::PathBuf;

use asset_model::config::{AssetSlug, AssetSlugBuilder, AssetTypeVariant};
use lazy_static::lazy_static;

use crate::{NAMESPACE_TEST, NAMESPACE_TEST_PATH};

/// Name of the "character_selection" ui asset.
pub const UI_CHARACTER_SELECTION_NAME: &str = "character_selection";

/// Name of the "loading" ui asset.
pub const UI_LOADING_NAME: &str = "loading";

lazy_static! {
    /// `PathBuf` to the "ui" asset directory.
    static ref UI_PATH: PathBuf =
        NAMESPACE_TEST_PATH.join(AssetTypeVariant::Ui.to_string());

    /// Slug of the "loading" ui asset.
    pub static ref UI_LOADING_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(UI_LOADING_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                UI_LOADING_NAME,
                e
            ))
    };

    /// `PathBuf` to the "loading" ui asset directory.
    pub static ref UI_LOADING_PATH: PathBuf = UI_PATH.join(UI_LOADING_NAME);

    /// Slug of the "character_selection" ui asset.
    pub static ref UI_CHARACTER_SELECTION_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(UI_CHARACTER_SELECTION_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                UI_CHARACTER_SELECTION_NAME,
                e
            ))
    };

    /// `PathBuf` to the "character_selection" ui asset directory.
    pub static ref UI_CHARACTER_SELECTION_PATH: PathBuf = UI_PATH.join(UI_CHARACTER_SELECTION_NAME);
}
