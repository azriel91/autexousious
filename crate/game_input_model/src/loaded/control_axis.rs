use strum_macros::{Display, EnumIter, EnumString};

/// Control axis input for characters.
///
/// This is not used in `InputConfig`, but as a logical representation
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumString, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ControlAxis {
    /// Up button.
    Up,
    /// Down button.
    Down,
    /// Left button.
    Left,
    /// Right button.
    Right,
}
