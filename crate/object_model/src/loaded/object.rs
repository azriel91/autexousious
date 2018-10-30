use std::collections::HashMap;

use config::object::SequenceId;
use loaded::{AnimatedComponentAnimation, AnimatedComponentDefault};

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId: SequenceId> {
    /// Handle to the default sprite sheet to use for the character.
    pub animation_defaults: Vec<AnimatedComponentDefault>,
    /// Handles to the animations that this object uses, keyed by sequence ID.
    pub animations: HashMap<SeqId, Vec<AnimatedComponentAnimation>>,
}
