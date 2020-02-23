use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

use crate::NetworkModeIndex;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `network_mode_selection select -s start_game`
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum NetworkModeSelectionEventArgs {
    /// Select event.
    Select {
        /// Index of the selection.
        index: NetworkModeIndex,
    },
    /// Close event.
    Close,
}
