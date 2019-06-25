#![allow(missing_debug_implementations)] // Needed for derived `EnumIter`

use strum_macros::{Display, EnumIter};

/// Game configuration types.
///
/// Allows compile-time checks for ensuring all configuration types are discovered.
#[derive(Clone, Copy, Debug, Display, EnumIter, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ConfigType {
    /// Things that can be interacted with in-game.
    Object,
    /// Playing field for objects.
    Map,
}
