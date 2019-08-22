use amethyst::assets::Asset;

use crate::loaded::Object;

/// Newtype for `Object`.
///
/// Due to orphan rules being too strict, we cannot implement:
///
/// ```rust,ignore
/// impl From<Object> for
///     Result<ProcessingState<Object>, Error>
/// ```
///
/// It would work if both `SequenceId` and `Object` reside within the same crate, but since each
/// object type's sequence ID will reside in its own crate &ndash; separate from `object_model`
/// &ndash; we have to use this workaround.
///
/// TODO: Orphan rules too strict pending <https://github.com/rust-lang/rfcs/issues/1856>
pub trait ObjectWrapper: Asset
where
    Self: Sized,
{
    /// Returns a new `ObjectWrapper` instance.
    fn new(object: Object) -> Self;

    /// Returns a reference to the inner `Object`.
    fn inner(&self) -> &Object;

    /// Returns a mutable reference to the inner `Object`.
    fn inner_mut(&mut self) -> &mut Object;
}
