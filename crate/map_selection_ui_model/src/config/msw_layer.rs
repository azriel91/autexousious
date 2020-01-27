use serde::{Deserialize, Serialize};

use crate::config::MswLayerName;

/// Keys for special handling of map selection widget layers.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[serde(deny_unknown_fields, untagged)]
pub enum MswLayer {
    /// Known map selection widget layer name.
    Name(MswLayerName),
    /// Arbitrary string.
    String(String),
}
