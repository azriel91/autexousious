use kinematic_model::config::PositionInit;
use serde::{Deserialize, Serialize};

/// Configuration for a character selection asset preview widget.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct CswDefinition {
    /// Position of the asset preview widget on screen.
    pub position: PositionInit,
}

impl AsRef<PositionInit> for CswDefinition {
    fn as_ref(&self) -> &PositionInit {
        &self.position
    }
}
