use std::fmt::Debug;
use std::fmt::Display;

use minterpolate::InterpolationPrimitive;

/// Sampler primitive for `ActiveHandle` animations.
///
/// Note that sprites can only ever be animated with `Step`, or a panic will occur.
#[derive(Clone, Copy, Debug, Display, PartialEq, Serialize, Deserialize)]
pub enum ActiveHandlePrimitive<I>
where
    I: Clone + Copy + Debug + Display + PartialEq,
{
    /// The handle to use.
    Handle(I),
}

impl<I> InterpolationPrimitive for ActiveHandlePrimitive<I>
where
    I: Debug + Display + Clone + Copy + PartialEq,
{
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
