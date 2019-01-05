use std::collections::HashMap;

use amethyst::ecs::{storage::DenseVecStorage, Component};
use derivative::Derivative;
use derive_new::new;

use crate::{
    config::object::SequenceId,
    loaded::{AnimatedComponentAnimation, AnimatedComponentDefault},
};

/// Represents an in-game object that has been loaded.
#[derive(Clone, Derivative, PartialEq, new)]
#[derivative(Debug)]
pub struct Object<SeqId>
where
    SeqId: SequenceId,
{
    /// Handle to the default sprite sheet to use for the object.
    pub animation_defaults: Vec<AnimatedComponentDefault>,
    /// Handles to the animations that this object uses, keyed by sequence ID.
    pub animations: HashMap<SeqId, Vec<AnimatedComponentAnimation>>,
}

impl<SeqId> Component for Object<SeqId>
where
    SeqId: SequenceId + 'static,
{
    type Storage = DenseVecStorage<Self>;
}
