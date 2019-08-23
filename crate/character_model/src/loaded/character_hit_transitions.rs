use amethyst::ecs::{storage::DenseVecStorage, Component};
use derive_new::new;
use sequence_model::loaded::SequenceId;
use specs_derive::Component;

/// Sequence to transition to when hit by another entity.
///
/// This is a hack to allow the `CharacterHitEffectSystem` to transition character sequences.
// TODO: Commonize Transition systems <https://gitlab.com/azriel91/autexousious/issues/157>
#[derive(Clone, Component, Copy, Debug, PartialEq, new)]
#[storage(DenseVecStorage)]
pub struct CharacterHitTransitions {
    /// Sequence ID to transition to when stun points are low.
    pub low_stun: SequenceId,
    /// Sequence ID to transition to when stun points are at a moderate level.
    pub mid_stun: SequenceId,
    /// Sequence ID to transition to when stun points are high.
    pub high_stun: SequenceId,
    /// Sequence ID to transition to when falling.
    pub falling: SequenceId,
}
