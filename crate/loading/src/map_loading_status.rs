use derivative::Derivative;

/// Status of map asset loading.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum MapLoadingStatus {
    /// Map asset loading is in progress.
    #[derivative(Default)]
    InProgress,
    /// Map asset loading is complete.
    Complete,
}
