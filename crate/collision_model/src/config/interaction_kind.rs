use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::config::Hit;

/// Type of collision -- hit, picking weapon, grabbing, and so on.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum InteractionKind {
    /// Collision happens on hit, and requires a cooldown period before the from
    /// entity can re-hit other entities.
    #[derivative(Default)]
    Hit(Hit),
}
