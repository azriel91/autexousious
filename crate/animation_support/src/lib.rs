#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides types to make it easier to work with Amethyst animations.

#[macro_use]
extern crate derive_new;
extern crate fnv;

pub use animation_data_set::AnimationDataSet;

mod animation_data_set;
