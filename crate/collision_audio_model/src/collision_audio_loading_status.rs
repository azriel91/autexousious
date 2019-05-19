use derivative::Derivative;

/// Status of Collision audio asset loading.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum CollisionAudioLoadingStatus {
    /// Collision audio asset loading has not started.
    #[derivative(Default)]
    NotStarted,
    /// Collision audio asset loading is in progress.
    InProgress,
    /// Collision audio asset loading is complete.
    Complete,
}
