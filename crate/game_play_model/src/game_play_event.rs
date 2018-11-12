/// Event signalling a change in game play state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePlayEvent {
    /// Returns to the menu.
    Return,
    /// Restarts the round.
    Restart,
    /// Pauses the round.
    Pause,
    /// Resumes the round.
    Resume,
    /// Signals the end of the round.
    End,
    /// Signals to go to the round statistics.
    EndStats,
}
