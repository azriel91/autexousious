use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_new::new;
use specs_derive::Component;

use crate::{
    config::SequenceId,
    loaded::{ControlTransitionHold, ControlTransitionPress},
};

/// Sequence to transition to on control input.
#[derive(Clone, Component, Copy, Debug, Derivative, PartialEq, Eq, new)]
#[derivative(Default)]
#[storage(VecStorage)]
pub enum ControlTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// No transition on control input.
    #[derivative(Default)]
    None,
    /// Transition to a specified sequence on control input press event.
    Press(ControlTransitionPress<SeqId>),
    /// Transition to a specified sequence on control input enabled state.
    Hold(ControlTransitionHold<SeqId>),
}
