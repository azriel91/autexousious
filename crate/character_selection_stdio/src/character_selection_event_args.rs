use game_input::ControllerId;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `character_selection select -c 1 -s default/heat`
/// * `character_selection deselect -c 1`
/// * `character_selection confirm`
//
// TODO: Pending <https://github.com/TeXitoi/structopt/issues/18>
// TODO: Update `StructOpt` to support automatic snake_case names.
#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum CharacterSelectionEventArgs {
    /// Select event.
    #[structopt(name = "select")]
    Select {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short = "c", long = "controller")]
        controller_id: ControllerId,
        /// Slug of the character or random, e.g. "default/heat", "random".
        #[structopt(short = "s", long = "selection")]
        selection: String,
    },
    /// Deselect event.
    #[structopt(name = "deselect")]
    Deselect {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short = "c", long = "controller")]
        controller_id: ControllerId,
    },
    /// Confirm event.
    #[structopt(name = "confirm")]
    Confirm,
}
