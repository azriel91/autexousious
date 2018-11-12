/// Event signalling a change in game play state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePlayEvent {
    /// Returns to the menu.
    Return,
    /// Restarts the round.
    Restart,
    /// Signals the end of the round.
    End,
}
