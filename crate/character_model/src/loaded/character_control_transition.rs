use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use sequence_model::loaded::{ControlTransition, ControlTransitionLike};
use specs_derive::Component;

use crate::config::{CharacterSequenceId, ControlTransitionRequirement};

/// Sequence to transition to on control input with requirements.
#[derive(Clone, Component, Debug, PartialEq, Eq, new)]
#[storage(VecStorage)]
pub struct CharacterControlTransition {
    /// Underlying `ControlTransition`.
    pub control_transition: ControlTransition<CharacterSequenceId>,
    /// Requirements for this transition to happen.
    pub control_transition_requirements: Vec<ControlTransitionRequirement>,
}

impl ControlTransitionLike<CharacterSequenceId> for CharacterControlTransition {
    fn control_transition(&self) -> &ControlTransition<CharacterSequenceId> {
        &self.control_transition
    }
}
