use std::path::PathBuf;

use asset_model::config::{AssetSlug, AssetSlugBuilder};
use lazy_static::lazy_static;
use object_type::ObjectType;

use crate::{NAMESPACE_TEST, OBJECT_PATH};

/// Name of the "square" energy asset.
pub const ENERGY_SQUARE_NAME: &str = "square";
/// File name of the white square sprites.
pub const ENERGY_SQUARE_SPRITE_NAME: &str = "white.png";

lazy_static! {
    /// `PathBuf` to the "objects" asset directory.
    static ref ENERGY_PATH: PathBuf =
        OBJECT_PATH.join(ObjectType::Energy.to_string());

    /// Slug of the "square" energy asset.
    pub static ref ENERGY_SQUARE_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_TEST.to_string())
            .name(ENERGY_SQUARE_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_TEST,
                ENERGY_SQUARE_NAME,
                e
            ))
    };

    /// `PathBuf` to the "square" energy asset directory.
    pub static ref ENERGY_SQUARE_PATH: PathBuf = ENERGY_PATH.join(ENERGY_SQUARE_NAME);
}
