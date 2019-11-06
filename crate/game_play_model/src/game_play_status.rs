use derivative::Derivative;

/// Status within the `GamePlayState`.
#[derive(Clone, Copy, Debug, Derivative, PartialEq)]
#[derivative(Default)]
pub enum GamePlayStatus {
    /// None,
    #[derivative(Default)]
    None,
    /// Round is in play.
    Playing,
    /// Round is paused.
    Paused,
    /// Round has ended.
    Ended,
}
