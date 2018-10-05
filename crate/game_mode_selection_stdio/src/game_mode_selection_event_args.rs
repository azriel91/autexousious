use game_mode_selection_model::GameModeIndex;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `map_selection select -s default/eruption`
//
// TODO: Pending <https://github.com/TeXitoi/structopt/issues/18>
// TODO: Update `StructOpt` to support automatic snake_case names.
#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum GameModeSelectionEventArgs {
    /// Select event.
    #[structopt(name = "select")]
    Select {
        /// Index of the selection.
        index: GameModeIndex,
    },
    /// Close event.
    #[structopt(name = "close")]
    Close,
}
