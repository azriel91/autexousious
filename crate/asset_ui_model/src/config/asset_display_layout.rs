use serde::{Deserialize, Serialize};

use crate::config::AssetDisplayGrid;

/// Layout to arrange assets to display.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum AssetDisplayLayout {
    /// Lays out assets in a grid.
    Grid(AssetDisplayGrid),
}
