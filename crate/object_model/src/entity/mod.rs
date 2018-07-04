//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::character::CharacterInput;
pub use self::kinematic::{Kinematics, Position, Velocity};
pub use self::object_status::ObjectStatus;
pub use self::object_status_update::ObjectStatusUpdate;

mod character;
mod kinematic;
mod object_status;
mod object_status_update;
