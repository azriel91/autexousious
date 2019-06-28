use amethyst::ecs::{storage::VecStorage, Component};
use derive_new::new;
use specs_derive::Component;

use crate::config::{SequenceId, TickTransition};

/// Sequence to transition to when the current sequence ends.
#[derive(Clone, Component, Copy, Debug, Default, PartialEq, new)]
#[storage(VecStorage)]
pub struct SequenceEndTransition<SeqId>
where
    SeqId: SequenceId,
{
    /// ID of the sequence to switch to after this one has completed.
    ///
    /// Note: This may not be immediately after the last frame of the sequence. For example, a
    /// character that is in mid-air should remain in the last frame until it lands on the ground.
    pub next: TickTransition<SeqId>,
}
