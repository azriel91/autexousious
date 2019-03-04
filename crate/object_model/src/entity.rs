//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::{
    character::RunCounter,
    grounding::Grounding,
    health_points::HealthPoints,
    kinematic::{Position, Velocity},
    mirrored::Mirrored,
};

mod character;
mod grounding;
mod health_points;
mod kinematic;
mod mirrored;
