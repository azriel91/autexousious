//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::character::{CharacterStatus, CharacterStatusUpdate, RunCounter};
pub use self::grounding::Grounding;
pub use self::health_points::HealthPoints;
pub use self::kinematic::{Kinematics, Position, Velocity};
pub use self::mirrored::Mirrored;
pub use self::object_status::ObjectStatus;
pub use self::object_status_update::ObjectStatusUpdate;
pub use self::sequence_status::SequenceStatus;

mod character;
mod grounding;
mod health_points;
mod kinematic;
mod mirrored;
mod object_status;
mod object_status_update;
mod sequence_status;
