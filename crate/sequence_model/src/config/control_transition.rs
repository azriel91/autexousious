use amethyst::ecs::{storage::VecStorage, Component};
use derivative::Derivative;
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs_derive::Component;

use crate::config::{ControlTransitionHold, ControlTransitionPress, SequenceId};

/// Sequence to transition to on control input.
#[derive(Clone, Component, Copy, Debug, Derivative, Deserialize, PartialEq, Eq, Serialize, new)]
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
