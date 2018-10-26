use minterpolate::InterpolationPrimitive;

use animation::CollisionFrameId;

/// Sampler primitive for CollisionFrame animations
/// Note that sprites can only ever be animated with `Step`, or a panic will occur.
#[derive(Debug, Display, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum CollisionFramePrimitive {
    /// The frame to use.
    Frame(CollisionFrameId),
}

impl InterpolationPrimitive for CollisionFramePrimitive {
    fn add(&self, _: &Self) -> Self {
        panic!("Cannot add CollisionFramePrimitive")
    }

    fn sub(&self, _: &Self) -> Self {
        panic!("Cannot sub CollisionFramePrimitive")
    }

    fn mul(&self, _: f32) -> Self {
        panic!("Cannot mul CollisionFramePrimitive")
    }

    fn dot(&self, _: &Self) -> f32 {
        panic!("Cannot dot CollisionFramePrimitive")
    }

    fn magnitude2(&self) -> f32 {
        panic!("Cannot magnitude2 CollisionFramePrimitive")
    }

    fn magnitude(&self) -> f32 {
        panic!("Cannot magnitude CollisionFramePrimitive")
    }

    fn normalize(&self) -> Self {
        panic!("Cannot normalize CollisionFramePrimitive")
    }
}
