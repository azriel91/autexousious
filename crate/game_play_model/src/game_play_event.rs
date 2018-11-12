/// Event signalling a change in game play state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePlayEvent {
    /// Stops the round.
    Cancel,
    /// Restarts the round.
    Restart,
    /// Signals the end of the round.
    End,
}
