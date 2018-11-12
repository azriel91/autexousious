/// Status within the `GamePlayState`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GamePlayStatus {
    /// Round is in play.
    Playing,
    /// Round is paused.
    Paused,
    /// Round has ended.
    Ended,
}
