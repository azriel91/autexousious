/// Event signalling a change in game play state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePlayEvent {
    /// Stop the round immediately.
    Cancel,
    /// Restart the round.
    Restart,
    /// The round has ended.
    End,
}
