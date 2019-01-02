use derive_deref::{Deref, DerefMut};
use object_model::{impl_processing_state_from_object, loaded::Object};

use crate::config::CharacterSequenceId;

/// Newtype for `Object<CharacterSequenceId>`.
///
/// Due to orphan rules being too strict, we cannot implement:
///
/// ```rust,ignore
/// impl From<Object<CharacterSequenceId>> for
///     Result<ProcessingState<Object<CharacterSequenceId>>, Error>
/// ```
///
/// It would work if both `CharacterSequenceId` and `Object` reside within the same crate, which was
/// the case before `character_model` was split from `object_model`.
#[derive(Debug, Deref, DerefMut)]
pub struct CharacterObject(pub Object<CharacterSequenceId>);

impl_processing_state_from_object!(CharacterObject);
