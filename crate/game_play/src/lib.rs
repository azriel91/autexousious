#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides the state and systems for game play.
//!
//! Note that game entities are spawned in the `GameLoadingState` provided by the `game_loading`
//! crate.








#[macro_use]
extern crate derivative;
#[macro_use]
extern crate derive_new;





#[macro_use]
extern crate log;



#[macro_use]
extern crate named_type_derive;




use shred;


use typename;
#[macro_use]
extern crate typename_derive;

pub(crate) use crate::game_play_bundle::GamePlayBundle;
pub use crate::game_play_state::GamePlayState;
pub(crate) use crate::system::{
    CharacterCollisionEffectSystem, CharacterGroundingSystem, CharacterKinematicsSystem,
    CharacterSequenceUpdateSystem, GamePlayEndDetectionSystem, GamePlayEndTransitionSystem,
    ObjectAnimationUpdateSystem, ObjectCollisionDetectionSystem, ObjectKinematicsUpdateSystem,
    ObjectTransformUpdateSystem,
};

mod game_play_bundle;
mod game_play_state;
mod system;
