/// Where a value lies in comparison to a range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Comparative {
    /// Below the range.
    Below,
    /// Within the range.
    Within,
    /// Above the range.
    Above,
}
