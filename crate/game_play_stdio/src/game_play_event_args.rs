/// Parameters to the mapper.
///
/// # Examples
///
/// * `game_play return`
/// * `game_play restart`
/// * `game_play end`
#[derive(Clone, Copy, Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum GamePlayEventArgs {
    /// Returns to the menu.
    Return,
    /// Restarts the round.
    Restart,
    /// Signals the end of the round.
    End,
}
