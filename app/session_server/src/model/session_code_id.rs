use std::fmt::{self, Display};

use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// ID so that we don't have to clone `SessionCode`.
#[derive(Clone, Copy, Debug, Default, Deref, DerefMut, Hash, PartialEq, Eq, new)]
pub struct SessionCodeId(pub u64);

impl Display for SessionCodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
