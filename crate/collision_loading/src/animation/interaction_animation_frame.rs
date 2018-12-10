use std::{fmt::Debug, hash::Hash};

use collision_model::config::{Interaction, InteractionFrame};

/// Information for a single animation frame.
///
/// Animation frames carry the following information:
///
/// * **Texture index:** Index of the material that holds the loaded collision sheet texture.
/// * **Collision index:** Index of the collision's texture offset coordinates.
/// * **Wait:** Number of ticks to wait before the animation switches to the next frame.
pub trait InteractionAnimationFrame: Clone + Debug + Hash + PartialEq + Eq {
    /// Returns the number of ticks to wait before the animation switches to the next frame.
    fn interactions(&self) -> Option<&Vec<Interaction>>;
    /// Returns the number of ticks to wait before the animation switches to the next frame.
    fn wait(&self) -> u32;
}

impl InteractionAnimationFrame for InteractionFrame {
    fn interactions(&self) -> Option<&Vec<Interaction>> {
        self.interactions.as_ref()
    }

    fn wait(&self) -> u32 {
        self.wait
    }
}
