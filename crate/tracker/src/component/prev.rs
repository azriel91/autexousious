use derive_deref::{Deref, DerefMut};
use derive_new::new;

/// Stores the previous value of a resource.
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, new)]
pub struct Prev<T>(pub T)
where
    T: Send + Sync;
