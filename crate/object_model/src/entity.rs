//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::{
    character::RunCounter,
    frame_index_clock::FrameIndexClock,
    grounding::Grounding,
    health_points::HealthPoints,
    kinematic::{Position, Velocity},
    mirrored::Mirrored,
    sequence_status::SequenceStatus,
};

mod character;
mod frame_index_clock;
mod grounding;
mod health_points;
mod kinematic;
mod mirrored;
mod sequence_status;
