use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use specs_derive::Component;

use crate::{
    config::SequenceId,
    loaded::{
        ControlTransitionDefault, ControlTransitionHold, ControlTransitionPress,
        ControlTransitionRelease,
    },
};

/// Sequence to transition to on control input.
#[derive(Clone, Component, Copy, Debug, PartialEq, Eq, new)]
#[storage(VecStorage)]
pub enum ControlTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// Transition to a specified sequence on control input press event.
    Press(ControlTransitionPress<SeqId>),
    /// Transition to a specified sequence on control input enabled state.
    Hold(ControlTransitionHold<SeqId>),
    /// Transition to a specified sequence on control input release event.
    Release(ControlTransitionRelease<SeqId>),
    /// Transition to a specified fallback sequence.
    Default(ControlTransitionDefault<SeqId>),
}
