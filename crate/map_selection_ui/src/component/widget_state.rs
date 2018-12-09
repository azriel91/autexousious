use derivative::Derivative;
use strum_macros::Display;

/// Map selection state of the `MapSelectionWidget`.
#[derive(Clone, Copy, Debug, Derivative, Display, PartialEq, Eq)]
#[derivative(Default)]
pub(crate) enum WidgetState {
    /// Map is being selected.
    #[derivative(Default)]
    MapSelect,
    /// Selection has been confirmed.
    Ready,
}
