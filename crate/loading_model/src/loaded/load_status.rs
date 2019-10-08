/// Status of a loading stage.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LoadStatus {
    /// Asset is queued for this load stage, but has not begun loading.
    Pending,
    /// Asset part is loading.
    Loading,
    /// Asset part has finished loading.
    Complete,
}
