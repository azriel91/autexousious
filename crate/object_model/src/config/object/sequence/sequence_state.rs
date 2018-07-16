/// States that indicate whether a sequence has just began,
#[derive(Clone, Copy, Debug, Derivative, PartialEq, Eq)]
#[derivative(Default)]
pub enum SequenceState {
    /// The sequence has just begun.
    #[derivative(Default)]
    Begin,
    /// The sequence began at least one tick ago, and has not yet reached the end.
    Ongoing,
    /// The sequence has ended.
    End,
}
