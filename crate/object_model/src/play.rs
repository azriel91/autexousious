//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::{
    charge_points::ChargePoints, grounding::Grounding, health_points::HealthPoints,
    mirrored::Mirrored, parent_object::ParentObject, skill_points::SkillPoints,
};

mod charge_points;
mod grounding;
mod health_points;
mod mirrored;
mod parent_object;
mod skill_points;
