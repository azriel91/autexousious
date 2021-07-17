/// Behaviour when input is received that would go beyond the limits of a row /
/// column of widgets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SiblingsBoundaryAction {
    /// Stop at the beginning / end of the row / column.
    Stop,
    // TODO: /// Wrap around to the beginning / end of the same row / column.
    // TODO: CycleSame,
    /// Wrap around to the beginning of the next row / column.
    CycleNext,
}
