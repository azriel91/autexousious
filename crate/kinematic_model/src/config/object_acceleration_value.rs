use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::config::ObjectAccelerationValueExpr;

/// Whether acceleration is applied once or continuously.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Serialize)]
#[derivative(Default)]
#[serde(deny_unknown_fields, rename_all = "snake_case", untagged)]
pub enum ObjectAccelerationValue {
    /// Acceleration is a constant.
    #[derivative(Default)]
    Const(f32),
    /// Acceleration is calculated using an expression.
    Expr(ObjectAccelerationValueExpr),
}
