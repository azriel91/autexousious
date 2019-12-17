//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::{grounding::Grounding, health_points::HealthPoints, skill_points::SkillPoints};

mod grounding;
mod health_points;
mod skill_points;
