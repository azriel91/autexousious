#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides an enum for types of in-game objects.

use strum_macros::{Display, EnumIter};

/// Types of in-game objects.
///
/// In-game objects are those that can be interacted with.
#[allow(missing_debug_implementations)] // Needed for derived `EnumIter`
#[derive(Clone, Copy, Debug, Display, EnumIter, Hash, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum ObjectType {
    /// Player or AI controllable objects.
    Character,
    /// Used in tests.
    #[cfg(feature = "test-support")]
    TestObject,
}
