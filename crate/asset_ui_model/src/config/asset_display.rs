use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};

use crate::config::AssetDisplayLayout;

/// Display sheet for available assets.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AssetDisplay {
    /// Position of the sheet.
    pub position: PositionInit,
    /// How to layout the available assets.
    pub layout: AssetDisplayLayout,
}
