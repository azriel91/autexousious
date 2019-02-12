use derivative::Derivative;

/// Status of asset loading.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum LoadingStatus {
    /// Asset loading is in progress.
    #[derivative(Default)]
    InProgress,
    /// Asset loading is complete.
    Complete,
}
