//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::{
    charge_points::ChargePoints,
    grounding::Grounding,
    health_points::HealthPoints,
    kinematic::{Position, Velocity},
    mirrored::Mirrored,
    skill_points::SkillPoints,
};

mod charge_points;
mod grounding;
mod health_points;
mod kinematic;
mod mirrored;
mod skill_points;
