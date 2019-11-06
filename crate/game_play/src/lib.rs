#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the state and systems for game play.
//!
//! Note that game entities are spawned in the `GameLoadingState` provided by the `game_loading`
//! crate.

pub use crate::{
    game_play_bundle::GamePlayBundle,
    game_play_state::GamePlayState,
    system::{
        CharacterHitEffectSystem, CharacterHitEffectSystemData, CharacterSequenceUpdateSystem,
        CharacterSequenceUpdateSystemData, FrameFreezeClockAugmentSystem,
        FrameFreezeClockAugmentSystemData, GamePlayEndDetectionSystem,
        GamePlayEndDetectionSystemData, GamePlayEndTransitionDelaySystem,
        GamePlayEndTransitionDelaySystemData, GamePlayEndTransitionSystem,
        GamePlayEndTransitionSystemData, GamePlayRemovalAugmentSystem,
        GamePlayRemovalAugmentSystemData, GamePlayStatusDisplaySystem,
        GamePlayStatusDisplaySystemData, GroundingFrictionSystem, GroundingFrictionSystemData,
        ObjectKinematicsUpdateSystem, ObjectKinematicsUpdateSystemData,
        ObjectTransformUpdateSystem, ObjectTransformUpdateSystemData,
        GAME_PLAY_END_TRANSITION_DELAY_DEFAULT,
    },
};

mod game_play_bundle;
mod game_play_state;
mod system;
