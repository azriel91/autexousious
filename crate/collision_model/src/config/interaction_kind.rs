use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::config::Impact;

/// Type of collision -- impact, picking weapon, grabbing, and so on.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Hash, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum InteractionKind {
    /// Collision happens on impact, and requires a cooldown period before the from entity can
    /// re-impact other entities.
    #[derivative(Default)]
    Impact(Impact),
}
