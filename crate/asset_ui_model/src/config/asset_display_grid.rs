use serde::{Deserialize, Serialize};
use ui_model_spi::config::Dimensions;

/// Lays out assets in a grid.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetDisplayGrid {
    /// Number of columns per row in the grid.
    pub column_count: usize,
    /// Width and height of each cell in this grid.
    pub cell_size: Dimensions,
}
