use std::{fmt::Debug, hash::Hash};

use shape_model::Volume;

use collision_model::config::BodyFrame;

/// Information for a single animation frame.
///
/// Animation frames carry the following information:
///
/// * **Texture index:** Index of the material that holds the loaded collision sheet texture.
/// * **Collision index:** Index of the collision's texture offset coordinates.
/// * **Wait:** Number of ticks to wait before the animation switches to the next frame.
pub trait BodyAnimationFrame: Clone + Debug + Hash + PartialEq + Eq {
    /// Returns the index of the collision's texture offset coordinates.
    fn body(&self) -> Option<&Vec<Volume>>;
    /// Returns the number of ticks to wait before the animation switches to the next frame.
    fn wait(&self) -> u32;
}

impl BodyAnimationFrame for BodyFrame {
    fn body(&self) -> Option<&Vec<Volume>> {
        self.body.as_ref()
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}
