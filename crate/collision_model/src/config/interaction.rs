use derive_new::new;
use serde::{Deserialize, Serialize};
use shape_model::Volume;

use crate::config::InteractionKind;

/// Effects of one object on another
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Eq, Serialize, new)]
#[serde(rename_all = "snake_case")]
pub struct Interaction {
    /// Effect behaviour of the collision -- impact, continuous, and so on.
    #[serde(flatten)]
    pub kind: InteractionKind,
    /// Effect volume.
    pub bounds: Vec<Volume>,
    /// Whether this will hit multiple objects. Defaults to `false`.
    #[serde(default)]
    pub multiple: bool,
}
