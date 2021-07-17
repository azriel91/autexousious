use derivative::Derivative;

/// Status of the user selecting a map.
///
/// Not to be confused with the Amethyst `State`, this is used to track whether
/// a map is chosen.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum MapSelectionStatus {
    /// The `MapSelection` is still waiting for confirmation.
    #[derivative(Default)]
    Pending,
    /// The `MapSelection` has been confirmed.
    Confirmed,
}
