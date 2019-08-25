use serde::{Deserialize, Serialize};

use crate::config::{GameObjectSequence, ObjectDefinition};

/// Components common to object types' definitions, associated with a sequence.
pub trait GameObjectDefinition {
    /// Sequence that this `GameObjectDefinition` uses.
    type GameObjectSequence: GameObjectSequence;

    /// Returns the `ObjectDefinition` for this `GameObjectDefinition`.
    fn object_definition(&self) -> &ObjectDefinition<Self::GameObjectSequence>
    where
        <Self::GameObjectSequence as GameObjectSequence>::SequenceName:
            for<'des> Deserialize<'des> + Serialize;
}
