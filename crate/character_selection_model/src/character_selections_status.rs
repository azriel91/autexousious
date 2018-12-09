use strum_macros::Display;

/// State of all character selections.
#[derive(Clone, Copy, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
pub enum CharacterSelectionsStatus {
    /// No players have joined.
    #[derivative(Default)]
    Waiting,
    /// Characters are being selected.
    CharacterSelect,
    /// All active selections have been confirmed.
    Confirmed,
    /// Player has signalled to move to the next state.
    Ready,
}
