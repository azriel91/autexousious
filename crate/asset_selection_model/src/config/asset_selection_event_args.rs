use game_input_model::ControllerId;
use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `asset_selection return`
/// * `asset_selection join -c 0`
/// * `asset_selection leave -c 0`
/// * `asset_selection switch -c 0 -s default/heat`
/// * `asset_selection select -c 0 -s default/heat`
/// * `asset_selection deselect -c 0`
/// * `asset_selection confirm`
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum AssetSelectionEventArgs {
    /// Signal to return from `AssetSelectionState`.
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
    /// Asset has been selected.
    Switch {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
        /// Slug of the asset or random, e.g. "default/heat", "random".
        #[structopt(short, long)]
        selection: String,
    },
    /// Asset has been selected.
    Select {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
        /// Slug of the asset or random, e.g. "default/heat", "random".
        #[structopt(short, long)]
        selection: String,
    },
    /// Asset has been deselected.
    Deselect {
        /// Controller ID.
        ///
        /// 0 for the first player, 1 for the second player, etcetera.
        #[structopt(short, long)]
        controller_id: ControllerId,
    },
    /// Asset selections have been confirmed.
    Confirm,
}
