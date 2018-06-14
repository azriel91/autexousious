use amethyst::{
    animation::{Animation, AnimationCommand, AnimationControlSet, AnimationSampling, EndControl},
    assets::Handle,
};
use std::hash::Hash;

#[derive(Debug)]
pub struct AnimationRunner;

impl AnimationRunner {
    pub fn start<SeqId: Copy + Eq + Hash + Send + Sync, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        sequence_id: &SeqId,
    ) {
        Self::internal_start(animation_set, animation_handle, sequence_id);
    }

    pub fn swap<SeqId: Copy + Eq + Hash + Send + Sync, T: AnimationSampling>(
        animation_set: &mut AnimationControlSet<SeqId, T>,
        animation_handle: &Handle<Animation<T>>,
        current_sequence_id: &SeqId,
        next_sequence_id: &SeqId,
    ) {
        // Abort the previous animation
        animation_set.abort(*current_sequence_id);

        // Start the next animation
        Self::internal_start(animation_set, animation_handle, next_sequence_id);
    }

    fn internal_start<SeqId: Copy + Eq + Hash + Send + Sync, T: AnimationSampling>(
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
