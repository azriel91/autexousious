use amethyst::{
    animation::{Animation, AnimationCommand, AnimationControlSet, AnimationSampling, EndControl},
    assets::Handle,
};

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
    /// * `sequence_id`: ID to track the animation control set.
    /// * `animation_set`: Animation control set to start.
    /// * `animation_handle`: Handle to the animation to include in the set.
    pub fn start<SeqId: PartialEq + Copy, T: AnimationSampling>(
        sequence_id: SeqId,
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
    ) {
        Self::internal_start(
            sequence_id,
            animation_set,
            animation_handle,
            EndControl::Stay,
        );
    }

    /// Starts and loops an animation control set.
    ///
    /// # Parameters
    ///
    /// * `sequence_id`: ID to track the animation control set.
    /// * `animation_set`: Animation control set to start.
    /// * `animation_handle`: Handle to the animation to include in the set.
    pub fn start_loop<SeqId: PartialEq + Copy, T: AnimationSampling>(
        sequence_id: SeqId,
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
    ) {
        Self::internal_start(
            sequence_id,
            animation_set,
            animation_handle,
            EndControl::Loop(None),
        );
    }

    /// Stops an existing animation control set and starts another.
    ///
    /// # Parameters
    ///
    /// * `current_sequence_id`: ID of the animation control set to stop.
    /// * `next_sequence_id`: ID to track the animation control set.
    /// * `animation_set`: Animation control set to start.
    /// * `animation_handle`: Handle to the animation to include in the set.
    pub fn swap<SeqId: PartialEq + Copy, T: AnimationSampling>(
        current_sequence_id: SeqId,
        next_sequence_id: SeqId,
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
    ) {
        animation_set.abort(current_sequence_id);

        // Start the next animation
        Self::internal_start(
            next_sequence_id,
            animation_set,
            animation_handle,
            EndControl::Stay,
        );
    }

    fn internal_start<SeqId: PartialEq + Copy, T: AnimationSampling>(
        sequence_id: SeqId,
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        end_control: EndControl,
    ) {
        animation_set.add_animation(
            sequence_id,
            &animation_handle,
            end_control,
            30., // Rate at which the animation plays
            AnimationCommand::Start,
        );
    }
}
