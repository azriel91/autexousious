#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes collision configuration into the loaded collision model.






#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use typename;
#[macro_use]
extern crate typename_derive;

pub use crate::animation::{
    BodyAnimationFrame, BodyAnimationLoader, BodyAnimationSequence, InteractionAnimationFrame,
    InteractionAnimationLoader, InteractionAnimationSequence,
};
pub use crate::collision_loading_bundle::CollisionLoadingBundle;
pub(crate) use crate::system::CollisionLoadingSystem;

mod animation;
mod collision_loading_bundle;
mod system;
