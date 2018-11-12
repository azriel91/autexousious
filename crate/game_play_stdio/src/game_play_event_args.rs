/// Parameters to the mapper.
///
/// # Examples
///
/// * `game_play cancel`
/// * `game_play restart`
/// * `game_play end`
#[derive(Clone, Copy, Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum GamePlayEventArgs {
    /// Stops the round.
    Cancel,
    /// Restarts the round.
    Restart,
    /// Signals the end of the round.
    End,
}
