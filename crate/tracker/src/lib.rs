#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Provides `Component`s to support detecting value changes.
//!
//! An example use case is input detection, where a `System` should react to input when a button is
//! pressed. The issue faced without value tracking is, you can read the state of the button as
//! pressed, but the `System` may be run multiple times before the user has released the button,
//! causing multiple actions to happen / rapid-fire when only one action is intended.

pub use crate::{component::Last, system::LastTrackerSystem};

mod component;
mod system;
