#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes collision configuration into the loaded collision model.

extern crate amethyst;
#[cfg(test)]
extern crate amethyst_test_support;
extern crate animation_support;
extern crate application;
#[cfg(test)]
extern crate assets_test;
extern crate collision_model;
#[cfg(test)]
#[macro_use]
extern crate derive_new;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
extern crate shape_model;

pub use animation::{
    CollisionAnimationFrame, CollisionAnimationLoader, CollisionAnimationSequence,
};

mod animation;
