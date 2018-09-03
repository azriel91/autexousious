use std::ops::{Deref, DerefMut};

use amethyst::ecs::prelude::*;
use named_type::NamedType;

/// Stores the last value of the component.
#[derive(Debug, NamedType)] // kcov-ignore
pub struct Last<T: Component + Clone + Send + Sync>(pub T);

impl<T> Component for Last<T>
where
    T: Component + Clone + Send + Sync,
{
    type Storage = DenseVecStorage<Self>;
}

impl<T> Deref for Last<T>
where
    T: Component + Clone + Send + Sync,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Last<T>
where
    T: Component + Clone + Send + Sync,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
