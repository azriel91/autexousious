use game_input::ControllerId;
use structopt_derive::StructOpt;

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
#[structopt(rename_all = "snake_case")]
pub enum CharacterSelectionEventArgs {
    /// Select event.
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
    Deselect {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short = "c", long = "controller")]
        controller_id: ControllerId,
    },
    /// Confirm event.
    Confirm,
}
