#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes collision configuration into the loaded collision model.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test;
extern crate animation_support;
extern crate application;
extern crate collision_model;
#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate shape_model;
extern crate typename;
#[macro_use]
extern crate typename_derive;

pub use animation::{
    BodyAnimationFrame, BodyAnimationLoader, BodyAnimationSequence, InteractionAnimationFrame,
    InteractionAnimationLoader, InteractionAnimationSequence,
};
pub use collision_loading_bundle::CollisionLoadingBundle;
pub(crate) use system::CollisionLoadingSystem;

mod animation;
mod collision_loading_bundle;
mod system;
