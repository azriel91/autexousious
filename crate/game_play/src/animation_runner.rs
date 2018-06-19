use amethyst::{
    animation::{Animation, AnimationCommand, AnimationControlSet, AnimationSampling, EndControl},
    assets::Handle,
};
use object_model::config::object::SequenceId;

#[derive(Debug)]
pub struct AnimationRunner;

impl AnimationRunner {
    pub fn start<SeqId: SequenceId, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        sequence_id: &SeqId,
    ) {
        Self::internal_start(animation_set, animation_handle, sequence_id);
    }

    pub fn swap<SeqId: SequenceId, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        current_sequence_id: &SeqId,
        next_sequence_id: &SeqId,
    ) {
        // Remove the previous animation
        //
        // There is a note saying this should be used with care:
        // <https://docs.rs/amethyst_animation/0.2.0/amethyst_animation/struct.AnimationControlSet.html#method.remove>
        //
        // However, if we use `#abort()`, the animation can freeze on the current animation instead
        // of moving to the next sequence's animation.
        animation_set.remove(*current_sequence_id);

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
            EndControl::Loop(None),
            30., // Rate at which the animation plays
            AnimationCommand::Start,
        );
    }
}
