use derivative::Derivative;
use strum_macros::Display;

/// Character selection state of the `CharacterSelectionWidget`.
#[derive(Clone, Copy, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
pub enum WidgetState {
    /// Player has not joined.
    #[derivative(Default)]
    Inactive,
    /// Character is being selected.
    CharacterSelect,
    /// Selection has been confirmed.
    Ready,
}
