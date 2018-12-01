//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::character::{CharacterStatus, CharacterStatusUpdate, RunCounter};
pub use self::grounding::Grounding;
pub use self::health_points::HealthPoints;
pub use self::kinematic::{Position, Velocity};
pub use self::mirrored::Mirrored;
pub use self::sequence_status::SequenceStatus;

mod character;
mod grounding;
mod health_points;
mod kinematic;
mod mirrored;
mod sequence_status;
