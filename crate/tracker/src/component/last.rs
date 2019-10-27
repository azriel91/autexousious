use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_deref::{Deref, DerefMut};
use derive_new::new;
use named_type::NamedType;
use named_type_derive::NamedType;

/// Stores the last value of the component.
#[derive(Debug, Deref, DerefMut, NamedType, new)] // kcov-ignore
pub struct Last<T>(pub T)
where
    T: Component + Clone + Send + Sync;

impl<T> Component for Last<T>
where
    T: Component + Clone + Send + Sync,
{
    type Storage = DenseVecStorage<Self>;
}
