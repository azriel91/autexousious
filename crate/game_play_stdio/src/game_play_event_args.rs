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
    /// Stop the round immediately.
    Cancel,
    /// Restart the round.
    Restart,
    /// The round has ended.
    End,
}
