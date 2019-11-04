use derive_deref::{Deref, DerefMut};
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;

/// Stores the previous value of a resource.
#[derive(Clone, Debug, Deref, DerefMut, NamedType, PartialEq, new)]
pub struct Prev<T>(pub T)
where
    T: Send + Sync;
