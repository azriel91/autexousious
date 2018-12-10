use serde::{Deserialize, Serialize};
use strum_macros::Display;

/// Axis to represent shape orientation.
#[derive(Clone, Copy, Debug, Deserialize, Display, Hash, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Axis {
    /// X axis.
    X,
    /// Y axis.
    Y,
    /// Z axis.
    Z,
}
