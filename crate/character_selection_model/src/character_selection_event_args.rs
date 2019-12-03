use game_input_model::ControllerId;
use structopt_derive::StructOpt;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `character_selection return`
/// * `character_selection join -c 0`
/// * `character_selection leave -c 0`
/// * `character_selection switch -c 0 -s default/heat`
/// * `character_selection select -c 0 -s default/heat`
/// * `character_selection deselect -c 0`
/// * `character_selection confirm`
#[derive(Clone, Debug, PartialEq, StructOpt)]
#[structopt(rename_all = "snake_case")]
pub enum CharacterSelectionEventArgs {
    /// Signal to return from `CharacterSelectionState`.
    Return,
    /// Player has joined / become active.
    Join {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
    },
    /// Player has left / become inactive.
    Leave {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
    },
    /// Character has been selected.
    Switch {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
        /// Slug of the character or random, e.g. "default/heat", "random".
        #[structopt(short, long)]
        selection: String,
    },
    /// Character has been selected.
    Select {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
        /// Slug of the character or random, e.g. "default/heat", "random".
        #[structopt(short, long)]
        selection: String,
    },
    /// Character has been deselected.
    Deselect {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
    },
    /// Character selections have been confirmed.
    Confirm,
}
