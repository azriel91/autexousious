use serde::{Deserialize, Serialize};

use crate::config::CswLayerName;

/// Keys for special handling of asset preview widget layers.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum CswLayer {
    /// Known asset preview widget layer name.
    Name(CswLayerName),
    /// Arbitrary string.
    String(String),
}
