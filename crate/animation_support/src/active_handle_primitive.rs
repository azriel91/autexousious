use amethyst::assets::Handle;

use minterpolate::InterpolationPrimitive;

/// Sampler primitive for `ActiveHandle` animations.
///
/// Note that handles can only ever be animated with `Step`, or a panic will occur.
#[derive(Clone, Debug, PartialEq)]
pub enum ActiveHandlePrimitive<T> {
    /// The handle to use.
    Handle(Handle<T>),
}

impl<T> InterpolationPrimitive for ActiveHandlePrimitive<T> {
    fn add(&self, _: &Self) -> Self {
        panic!("Cannot add ActiveHandlePrimitive")
    }

    fn sub(&self, _: &Self) -> Self {
        panic!("Cannot sub ActiveHandlePrimitive")
    }

    fn mul(&self, _: f32) -> Self {
        panic!("Cannot mul ActiveHandlePrimitive")
    }

    fn dot(&self, _: &Self) -> f32 {
        panic!("Cannot dot ActiveHandlePrimitive")
    }

    fn magnitude2(&self) -> f32 {
        panic!("Cannot magnitude2 ActiveHandlePrimitive")
    }

    fn magnitude(&self) -> f32 {
        panic!("Cannot magnitude ActiveHandlePrimitive")
    }

    fn normalize(&self) -> Self {
        panic!("Cannot normalize ActiveHandlePrimitive")
    }
}
