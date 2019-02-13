use derivative::Derivative;

/// Status of object asset loading.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum ObjectLoadingStatus {
    /// Object asset loading is in progress.
    #[derivative(Default)]
    InProgress,
    /// Object asset loading is complete.
    Complete,
}
