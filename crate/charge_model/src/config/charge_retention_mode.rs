use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// How charge is retained when no longer charging.
#[derive(Clone, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
pub enum ChargeRetentionMode {
    /// Charge is retained until spent.
    #[derivative(Default)]
    Forever,
    /// Charge is never retained, if there is no transition, then it is simply lost.
    Never,
    /// Charge is lost over time.
    Lossy {
        /// Number of ticks to wait between charge decrements.
        delay: usize,
    },
    /// Charge is reset if it is not used / added to in the specified duration.
    Reset {
        /// Number of ticks to wait before resetting the charge tracker.
        delay: usize,
    },
}
