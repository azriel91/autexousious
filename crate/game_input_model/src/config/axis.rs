use derivative::Derivative;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Control axes for objects.
#[derive(
    Clone,
    Copy,
    Debug,
    Derivative,
    Display,
    Deserialize,
    EnumIter,
    EnumString,
    Hash,
    PartialEq,
    Eq,
    Serialize,
)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Axis {
    /// X axis, positive is to the right, negative is to the left.
    #[derivative(Default)]
    X,
    /// Z axis, positive is downwards, negative is upwards.
    Z,
}
