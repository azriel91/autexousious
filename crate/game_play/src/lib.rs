#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides the state and systems for game play.
//!
//! Note that game entities are spawned in the `GameLoadingState` provided by the `game_loading`
//! crate.

pub use crate::game_play_state::GamePlayState;
pub(crate) use crate::{
    game_play_bundle::GamePlayBundle,
    system::{
        CharacterHitEffectSystem, CharacterSequenceUpdateSystem, FrameFreezeClockAugmentSystem,
        GamePlayEndDetectionSystem, GamePlayEndTransitionSystem, GamePlayRemovalAugmentSystem,
        GroundingFrictionSystem, ObjectKinematicsUpdateSystem, ObjectTransformUpdateSystem,
    },
};

mod game_play_bundle;
mod game_play_state;
mod system;
