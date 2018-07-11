use amethyst::{
    animation::{Animation, AnimationCommand, AnimationControlSet, AnimationSampling, EndControl},
    assets::Handle,
};
use object_model::config::object::SequenceId;

/// Starts, stops, and swaps animation control sets.
///
/// The animation control sets are generic, so they are not limited to `Material` (texture) animations.
#[derive(Debug)]
pub struct AnimationRunner;

impl AnimationRunner {
    /// Starts an animation control set.
    ///
    /// # Parameters
    ///
    /// * `animation_set`: Animation control set to start.
    /// * `animation_handle`: Handle to the animation to include in the set.
    /// * `sequence_id`: ID to track the animation control set.
    pub fn start<SeqId: SequenceId, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        sequence_id: &SeqId,
    ) {
        Self::internal_start(animation_set, animation_handle, sequence_id);
    }

    /// Stops an existing animation control set and starts another.
    ///
    /// # Parameters
    ///
    /// * `animation_set`: Animation control set to start.
    /// * `animation_handle`: Handle to the animation to include in the set.
    /// * `current_sequence_id`: ID of the animation control set to stop.
    /// * `next_sequence_id`: ID to track the animation control set.
    pub fn swap<SeqId: SequenceId, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        current_sequence_id: &SeqId,
        next_sequence_id: &SeqId,
    ) {
        animation_set.abort(*current_sequence_id);

        // Start the next animation
        Self::internal_start(animation_set, animation_handle, next_sequence_id);
    }

    fn internal_start<SeqId: SequenceId, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        sequence_id: &SeqId,
    ) {
        animation_set.add_animation(
            *sequence_id,
            &animation_handle,
            EndControl::Stay,
            30., // Rate at which the animation plays
            AnimationCommand::Start,
        );
    }
}
