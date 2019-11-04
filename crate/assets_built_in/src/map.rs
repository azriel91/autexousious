use asset_model::config::{AssetSlug, AssetSlugBuilder};
use background_model::config::BackgroundDefinition;
use lazy_static::lazy_static;
use map_model::config::{MapBounds, MapDefinition, MapHeader};

use crate::NAMESPACE_BUILT_IN;

/// Name of the "blank" map asset.
pub const MAP_BLANK_NAME: &str = "blank";

lazy_static! {
    /// Slug of the "blank" map asset.
    pub static ref MAP_BLANK_SLUG: AssetSlug = {
        AssetSlugBuilder::default()
            .namespace(NAMESPACE_BUILT_IN.to_string())
            .name(MAP_BLANK_NAME.to_string())
            .build()
            .unwrap_or_else(|e| panic!(
                "Expected `{}/{}` asset slug to build. Error: \n\n```{}\n```\n",
                NAMESPACE_BUILT_IN,
                MAP_BLANK_NAME,
                e
            ))
    };

    /// Built-in blank map.
    pub static ref MAP_DEFINITION_BLANK: MapDefinition = {
        let (width, height, depth) = (800, 600, 200);
        let bounds = MapBounds::new(0, 0, 0, width as u32, height as u32 - depth, depth);
        let header = MapHeader::new("Blank Screen".to_string(), bounds);
        MapDefinition::new(header, BackgroundDefinition::default())
    };
}
