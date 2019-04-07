use sequence_model::config::SequenceId;

use crate::config::ObjectDefinition;

/// Components common to object types' definitions, parameterized by sequence ID.
pub trait GameObjectDefinition {
    /// Sequence ID that this `GameObjectDefinition` uses.
    type SequenceId: SequenceId;

    /// Returns the `ObjectDefinition` for this `GameObjectDefinition`.
    fn object_definition(&self) -> &ObjectDefinition<Self::SequenceId>;
}
