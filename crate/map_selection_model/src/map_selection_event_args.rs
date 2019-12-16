use serde::{Deserialize, Serialize};
use structopt_derive::StructOpt;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `map_selection return`
/// * `map_selection select -s default/eruption`
/// * `map_selection deselect`
/// * `map_selection confirm`
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, StructOpt)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
#[structopt(rename_all = "snake_case")]
pub enum MapSelectionEventArgs {
    /// Return from map selection state.
    Return,
    /// Select a map.
    Select {
        /// Slug of the map or random, e.g. "default/eruption", "random".
        #[structopt(short, long)]
        selection: String,
    },
    /// Deselect the map.
    Deselect,
    /// Confirm map selection.
    Confirm,
}
