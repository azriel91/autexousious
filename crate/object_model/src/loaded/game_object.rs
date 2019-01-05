use amethyst::assets::Handle;

use crate::{
    config::object::SequenceId,
    loaded::{ObjectWrapper, SequenceEndTransitions},
};

/// Components common to object types, parameterized by sequence ID.
pub trait GameObject {
    /// Sequence ID that this `GameObject` uses.
    type SequenceId: SequenceId;
    /// Newtype wrapper for `Object<SequenceId>`.
    type ObjectWrapper: ObjectWrapper<SequenceId = Self::SequenceId>;

    /// Returns the handle to the loaded `Object` for this `GameObject`.
    fn object_handle(&self) -> &Handle<Self::ObjectWrapper>;
    /// Returns the sequence end transitions for this `GameObject`.
    fn sequence_end_transitions(&self) -> &SequenceEndTransitions<Self::SequenceId>;
}
