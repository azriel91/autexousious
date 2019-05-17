use derivative::Derivative;

/// Status of UI audio asset loading.
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum UiAudioLoadingStatus {
    /// UI audio asset loading has not started.
    #[derivative(Default)]
    NotStarted,
    /// UI audio asset loading is in progress.
    InProgress,
    /// UI audio asset loading is complete.
    Complete,
}
