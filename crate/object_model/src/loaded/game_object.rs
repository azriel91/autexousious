use crate::{
    config::object::SequenceId,
    loaded::{ObjectHandle, SequenceEndTransitions},
};

/// Components common to object types, parameterized by sequence ID.
///
/// # Examples
///
/// The struct this is used on **must** have both the `object_handle` and `sequence_end_transitions`
/// fields.
///
/// ```rust
/// use object_model::{
///     config::object::CharacterSequenceId,
///     loaded::{ObjectHandle, GameObject, SequenceEndTransitions},
/// };
/// use object_model_derive::GameObject;
///
/// /// Represents an in-game character that has been loaded.
/// #[derive(Clone, Debug, GameObject)]
/// pub struct Character {
///     /// Handle to loaded object data.
///     pub object_handle: ObjectHandle<CharacterSequenceId>,
///     /// Component sequence transitions when a sequence ends.
///     pub sequence_end_transitions: SequenceEndTransitions<CharacterSequenceId>,
/// }
/// ```
pub trait GameObject<SeqId>
where
    SeqId: SequenceId + 'static,
{
    /// Returns the handle to the loaded `Object` for this `GameObject`.
    fn object_handle(&self) -> &ObjectHandle<SeqId>;
    /// Returns the sequence end transitions for this `GameObject`.
    fn sequence_end_transitions(&self) -> &SequenceEndTransitions<SeqId>;
}
