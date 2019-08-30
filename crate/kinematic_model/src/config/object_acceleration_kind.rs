use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Whether acceleration is applied once or continuously.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ObjectAccelerationKind {
    #[derivative(Default)]
    /// Acceleration is applied as long as the object is on this frame.
    Continuous,
    /// Acceleration is applied only at the beginning of this frame.
    Once,
}
