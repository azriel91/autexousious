use game_mode_selection_model::GameModeIndex;
use structopt_derive::StructOpt;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `game_mode_selection select -s start_game`
#[derive(Clone, Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum GameModeSelectionEventArgs {
    /// Select event.
    Select {
        /// Index of the selection.
        index: GameModeIndex,
    },
    /// Close event.
    Close,
}
