use std::marker::PhantomData;

use amethyst::{
    animation::{Animation, AnimationControlSet, AnimationSampling},
    assets::Handle,
};

use crate::AnimationRunner;

/// Starts, stops, and swaps multiple animation control sets.
#[derive(Debug)]
pub struct MultiAnimationRunner<SeqId, T>(PhantomData<(SeqId, T)>);

macro_rules! impl_multi_runner {
    ($(
        $Tuple:ident {
            $(($idx:tt) -> $T:ident)+
        }
    )+) => {
        $(
            impl<SeqId, $($T,)+> MultiAnimationRunner<SeqId, ($($T,)+)>
            where
                SeqId: PartialEq + Copy,
                $($T: AnimationSampling,)+ {
                /// Starts multiple animation control sets.
                ///
                /// # Parameters
                ///
                /// * `sequence_id`: ID to track the animation control sets.
                /// * `animation_sets`: Animation control sets to start.
                /// * `animation_handle`: Handle to the animation to include in the set.
                pub fn start(
                    sequence_id: SeqId,
                    animation_sets: ($(&mut AnimationControlSet<SeqId, $T>,)+),
                    animation_handles: ($(&Handle<Animation<$T>>,)+),
                ) {
                    $(
                        AnimationRunner::start(
                            sequence_id,
                            animation_sets.$idx,
                            animation_handles.$idx
                        );
                    )+
                }

                /// Starts and loops multiple animation control sets.
                ///
                /// # Parameters
                ///
                /// * `sequence_id`: ID to track the animation control sets.
                /// * `animation_set`: Animation control set to start.
                /// * `animation_handle`: Handle to the animation to include in the set.
                pub fn start_loop(
                    sequence_id: SeqId,
                    animation_sets: ($(&mut AnimationControlSet<SeqId, $T>,)+),
                    animation_handles: ($(&Handle<Animation<$T>>,)+),
                ) {
                    $(
                        AnimationRunner::start_loop(
                            sequence_id,
                            animation_sets.$idx,
                            animation_handles.$idx
                        );
                    )+
                }

                /// Stops existing animation control sets and starts others.
                ///
                /// # Parameters
                ///
                /// * `current_sequence_id`: ID of the animation control sets to stop.
                /// * `next_sequence_id`: ID to track the animation control sets.
                /// * `animation_set`: Animation control set to start.
                /// * `animation_handle`: Handle to the animation to include in the set.
                pub fn swap(
                    current_sequence_id: SeqId,
                    next_sequence_id: SeqId,
                    animation_sets: ($(&mut AnimationControlSet<SeqId, $T>,)+),
                    animation_handles: ($(&Handle<Animation<$T>>,)+),
                ) {
                    $(
                        AnimationRunner::swap(
                            current_sequence_id,
                            next_sequence_id,
                            animation_sets.$idx,
                            animation_handles.$idx
                        );
                    )+
                }
            }
        )+
    };
}

impl_multi_runner! {
    Tuple1 {
        (0) -> A
    }
    Tuple2 {
        (0) -> A
        (1) -> B
    }
    Tuple3 {
        (0) -> A
        (1) -> B
        (2) -> C
    }
    Tuple4 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
    }
    Tuple5 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
    }
    Tuple6 {
        (0) -> A
        (1) -> B
        (2) -> C
        (3) -> D
        (4) -> E
        (5) -> F
    }
}
