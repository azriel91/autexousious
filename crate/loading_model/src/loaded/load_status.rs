/// Status of a loading stage.
///
/// This is intended to be used alongside `LoadStage` to track the loading
/// status of an asset part.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LoadStatus {
    /// Asset is queued for a given load stage, but has not begun loading.
    Queued,
    /// Asset part is loading.
    InProgress,
    /// Asset part has finished loading.
    Complete,
}
