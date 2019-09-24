#![allow(missing_debug_implementations)] // Needed for derived `EnumIter`

use object_type::ObjectType;
use strum_macros::{Display, EnumDiscriminants, EnumIter};

/// Game configuration types.
///
/// Allows compile-time checks for ensuring all configuration types are discovered.
#[derive(Clone, Copy, Debug, EnumDiscriminants, Hash, PartialEq, Eq)]
#[strum_discriminants(
    derive(Display, EnumIter, Hash),
    name(AssetTypeVariants),
    strum(serialize_all = "snake_case")
)]
pub enum AssetType {
    /// Things that can be interacted with in-game.
    Object(ObjectType),
    /// Playing field for objects.
    Map,
}
