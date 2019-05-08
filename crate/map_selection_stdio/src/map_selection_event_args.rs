use structopt_derive::StructOpt;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `map_selection select -s default/eruption`
#[derive(Clone, Debug, PartialEq, StructOpt)]
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
