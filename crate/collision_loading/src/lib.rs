#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes collision configuration into the loaded collision model.

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

pub(crate) use crate::system::CollisionLoadingSystem;
pub use crate::{
    animation::{
        BodyAnimationLoader, InteractionAnimationFrame, InteractionAnimationLoader,
        InteractionAnimationSequence,
    },
    collision_loading_bundle::CollisionLoadingBundle,
};

mod animation;
mod collision_loading_bundle;
mod system;
