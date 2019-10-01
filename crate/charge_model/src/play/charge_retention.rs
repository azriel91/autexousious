use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use serde::{Deserialize, Serialize};

use crate::{config::ChargeRetentionMode, play::ChargeRetentionClock};

/// How charge is retained when no longer charging.
#[derive(Clone, Component, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Serialize)]
#[derivative(Default)]
#[serde(rename_all = "snake_case")]
#[storage(VecStorage)]
pub enum ChargeRetention {
    /// Charge is retained until spent.
    #[derivative(Default)]
    Forever,
    /// Charge is never retained, if there is no transition, then it is simply lost.
    Never,
    /// Charge is lost over time.
    Lossy(ChargeRetentionClock),
    /// Charge is reset if it is not used / added to in the specified duration.
    Reset(ChargeRetentionClock),
}

impl From<ChargeRetentionMode> for ChargeRetention {
    fn from(charge_retention_mode: ChargeRetentionMode) -> Self {
        match charge_retention_mode {
            ChargeRetentionMode::Forever => ChargeRetention::Forever,
            ChargeRetentionMode::Never => ChargeRetention::Never,
            ChargeRetentionMode::Lossy { delay } => {
                ChargeRetention::Lossy(ChargeRetentionClock::new(delay))
            }
            ChargeRetentionMode::Reset { delay } => {
                ChargeRetention::Reset(ChargeRetentionClock::new(delay))
            }
        }
    }
}
