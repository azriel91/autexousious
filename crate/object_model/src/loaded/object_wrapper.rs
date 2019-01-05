use amethyst::assets::Asset;

use crate::{config::object::SequenceId, loaded::Object};

/// Newtype for `Object<SequenceId>`.
///
/// Due to orphan rules being too strict, we cannot implement:
///
/// ```rust,ignore
/// impl From<Object<SequenceId>> for
///     Result<ProcessingState<Object<SequenceId>>, Error>
/// ```
///
/// It would work if both `SequenceId` and `Object` reside within the same crate, but since each
/// object type's sequence ID will reside in its own crate &ndash; separate from `object_model`
/// &ndash; we have to use this workaround.
///
/// TODO: Orphan rules too strict pending <https://github.com/rust-lang/rfcs/issues/1856>
pub trait ObjectWrapper: Asset<Data = Self>
where
    Self: Sized,
{
    /// Sequence ID of the `Object<SeqId>`.
    type SequenceId: SequenceId;

    /// Returns a new `ObjectWrapper` instance.
    fn new(object: Object<Self::SequenceId>) -> Self;

    /// Returns a reference to the inner `Object<SequenceId>`.
    fn inner(&self) -> &Object<Self::SequenceId>;

    /// Returns a mutable reference to the inner `Object<SequenceId>`.
    fn inner_mut(&mut self) -> &mut Object<Self::SequenceId>;
}
