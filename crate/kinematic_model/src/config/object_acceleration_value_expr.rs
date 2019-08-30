use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::config::ObjectAccelerationValueMultiplier;

/// Expression to calculate acceleration value.
///
/// Strictly speaking this isn't an expression.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize, new)]
#[serde(deny_unknown_fields, default)]
pub struct ObjectAccelerationValueExpr {
    /// Indicates the attribute to use to multiply with the acceleration value.
    pub multiplier: ObjectAccelerationValueMultiplier,
    /// Acceleration value to be multiplied.
    pub value: f32,
}
