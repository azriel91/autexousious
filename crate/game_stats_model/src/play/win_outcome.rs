use derivative::Derivative;
use team_model::play::Team;

/// Outcome of a round.
#[derive(Clone, Copy, Debug, Derivative, PartialEq)]
#[derivative(Default)]
pub enum WinOutcome {
    /// There is currently no outcome.
    #[derivative(Default)]
    None,
    /// A team has won the round.
    WinLoss {
        /// Team that won the round.
        winning_team: Team,
    },
    /// The round ended in a draw.
    Draw,
}
