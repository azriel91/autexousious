use derive_new::new;

use crate::play::WinOutcome;

/// The win/loss information of a match.
#[derive(Clone, Copy, Debug, Default, PartialEq, new)]
pub struct WinStatus {
    /// The outcome, whether it was a win-loss, or a draw.
    pub outcome: WinOutcome,
}
