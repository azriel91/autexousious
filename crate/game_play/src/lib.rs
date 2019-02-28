#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the state and systems for game play.
//!
//! Note that game entities are spawned in the `GameLoadingState` provided by the `game_loading`
//! crate.

pub use crate::game_play_state::GamePlayState;
pub(crate) use crate::{
    game_play_bundle::GamePlayBundle,
    system::{
        CharacterCollisionEffectSystem, CharacterGroundingSystem, CharacterKinematicsSystem,
        CharacterSequenceUpdateSystem, GamePlayEndDetectionSystem, GamePlayEndTransitionSystem,
        ObjectCollisionDetectionSystem, ObjectFrameComponentUpdateSystem,
        ObjectKinematicsUpdateSystem, ObjectSequenceUpdateEvent, ObjectSequenceUpdateSystem,
        ObjectTransformUpdateSystem,
    },
};

mod game_play_bundle;
mod game_play_state;
mod system;
