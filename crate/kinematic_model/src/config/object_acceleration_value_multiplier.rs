use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Whether acceleration is applied once or continuously.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ObjectAccelerationValueMultiplier {
    /// The value should be used as is.
    #[derivative(Default)]
    One,
    /// Multiply the value by the x axis input.
    XAxis,
    /// Multiply the value by the z axis input.
    ZAxis,
}
