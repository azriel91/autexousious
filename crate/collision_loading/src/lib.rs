#![deny(missing_docs)] // kcov-ignore
#![deny(missing_debug_implementations)]

//! Processes collision configuration into the loaded collision model.

pub use crate::collision_loading_bundle::CollisionLoadingBundle;
pub(crate) use crate::system::CollisionLoadingSystem;

mod collision_loading_bundle;
mod system;
