use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use input_reaction_model::loaded::InputReaction;

use crate::config::CharacterIrr;

/// Sequence to transition to on control input with requirements.
#[derive(Clone, Component, Debug, PartialEq, new)]
#[storage(VecStorage)]
pub struct CharacterInputReaction {
    /// Underlying `InputReaction`.
    pub input_reaction: InputReaction,
    /// Requirements for this transition to happen.
    pub requirement: CharacterIrr,
}

impl AsRef<InputReaction> for CharacterInputReaction {
    fn as_ref(&self) -> &InputReaction {
        &self.input_reaction
    }
}

impl AsRef<CharacterIrr> for CharacterInputReaction {
    fn as_ref(&self) -> &CharacterIrr {
        &self.requirement
    }
}
