use structopt_derive::StructOpt;

/// Parameters to the mapper.
///
/// # Examples
///
/// * `map_selection select -s default/eruption`
#[derive(Clone, Debug, PartialEq, StructOpt)]
pub enum MapSelectionEventArgs {
    /// Select event.
    #[structopt(name = "select")]
    Select {
        /// Slug of the map or random, e.g. "default/eruption", "random".
        #[structopt(short = "s", long = "selection")]
        selection: String,
    },
}
