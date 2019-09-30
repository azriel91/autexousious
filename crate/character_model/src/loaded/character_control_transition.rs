use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use sequence_model::loaded::{ControlTransition, ControlTransitionLike};

use crate::config::ControlTransitionRequirement;

/// Sequence to transition to on control input with requirements.
#[derive(Clone, Component, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct CharacterControlTransition {
    /// Underlying `ControlTransition`.
    pub control_transition: ControlTransition,
    /// Requirements for this transition to happen.
    pub control_transition_requirements: Vec<ControlTransitionRequirement>,
}

impl ControlTransitionLike for CharacterControlTransition {
    fn control_transition(&self) -> &ControlTransition {
        &self.control_transition
    }
}
