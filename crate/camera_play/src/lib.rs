#![deny(missing_debug_implementations, missing_docs)] // kcov-ignore

//! Provides camera logic during game play.

pub use crate::{
    camera_component_storages::CameraComponentStorages,
    camera_creator::CameraCreator,
    camera_creator_resources::CameraCreatorResources,
    system::{CameraTrackingSystem, CameraVelocitySystem},
};

mod camera_component_storages;
mod camera_creator;
mod camera_creator_resources;
mod system;
