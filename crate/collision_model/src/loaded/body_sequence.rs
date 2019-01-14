use amethyst::{
    assets::{Asset, Handle, ProcessingState},
    ecs::storage::VecStorage,
    Error,
};
use derive_new::new;

use crate::config::BodyFrame;

/// Sequence for volumes that can be hit.
#[derive(Clone, Debug, Default, PartialEq, new)]
pub struct BodySequence {
    /// Handles to the frames in this sequence.
    pub frames: Vec<Handle<BodyFrame>>,
}

impl Asset for BodySequence {
    const NAME: &'static str = "collision_model::loaded::BodySequence";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<BodySequence> for Result<ProcessingState<BodySequence>, Error> {
    fn from(sequence: BodySequence) -> Result<ProcessingState<BodySequence>, Error> {
        Ok(ProcessingState::Loaded(sequence))
    }
}
