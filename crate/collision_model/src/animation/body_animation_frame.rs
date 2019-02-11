use std::fmt::Debug;

use shape_model::Volume;

use crate::config::BodyFrame;

/// Information for a single animation frame.
pub trait BodyAnimationFrame: Clone + Debug + PartialEq + Eq {
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
