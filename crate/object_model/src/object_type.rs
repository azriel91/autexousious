#![allow(missing_debug_implementations)] // Needed for derived `EnumIter`

/// Types of in-game objects.
///
/// In-game objects are those that can be interacted with.
#[derive(Clone, Copy, Debug, Display, EnumIter, Hash, PartialEq, Eq)]
pub enum ObjectType {
    /// Player or AI controllable objects.
    Character,
}
