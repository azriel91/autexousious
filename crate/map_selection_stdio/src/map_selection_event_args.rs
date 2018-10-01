/// Parameters to the mapper.
///
/// # Examples
///
/// * `map_selection select -s default/eruption`
//
// TODO: Pending <https://github.com/TeXitoi/structopt/issues/18>
// TODO: Update `StructOpt` to support automatic snake_case names.
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
