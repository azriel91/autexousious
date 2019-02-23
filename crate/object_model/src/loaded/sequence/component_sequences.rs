use derive_deref::{Deref, DerefMut};
use derive_new::new;

use crate::loaded::ComponentSequence;

/// Newtype for `Vec<ComponentSequence>`.
#[derive(Clone, Debug, Default, Deref, DerefMut, PartialEq, new)]
pub struct ComponentSequences(
    /// The underlying vector.
    pub Vec<ComponentSequence>,
);
