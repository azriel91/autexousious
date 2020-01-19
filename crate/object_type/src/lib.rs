#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides an enum for types of in-game objects.

use enum_variant_type::EnumVariantType;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Types of in-game objects.
///
/// In-game objects are those that can be interacted with.
#[derive(Clone, Copy, Debug, Display, EnumIter, EnumVariantType, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ObjectType {
    /// Player or AI controllable objects.
    #[evt(derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Serialize))]
    Character,
    /// Energy / aura / spark effects.
    #[evt(skip)]
    Energy,
    /// Used in tests.
    #[evt(skip)]
    TestObject,
}
