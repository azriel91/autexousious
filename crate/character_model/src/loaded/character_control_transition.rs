use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use sequence_model::loaded::InputReaction;

use crate::config::ControlTransitionRequirement;

/// Sequence to transition to on control input with requirements.
#[derive(Clone, Component, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct CharacterControlTransition {
    /// Underlying `InputReaction`.
    pub input_reaction: InputReaction,
    /// Requirements for this transition to happen.
    pub control_transition_requirements: Vec<ControlTransitionRequirement>,
}

impl AsRef<InputReaction> for CharacterControlTransition {
    fn as_ref(&self) -> &InputReaction {
        &self.input_reaction
    }
}
