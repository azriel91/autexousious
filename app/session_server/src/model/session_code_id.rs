use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// ID so that we don't have to clone `SessionCode`.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Hash, PartialEq, Eq, new)]
pub struct SessionCodeId(pub u64);
