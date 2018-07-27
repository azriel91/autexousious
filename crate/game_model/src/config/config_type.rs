#![allow(missing_debug_implementations)] // Needed for derived `EnumIter`

/// Game configuration types.
///
/// Allows compile-time checks for ensuring all configuration types are discovered.
#[derive(Clone, Copy, Debug, Display, EnumIter, PartialEq, Eq, Hash)]
pub enum ConfigType {
    /// Things that can be interacted with in-game.
    Object,
    /// Playing field for objects.
    Map,
}
