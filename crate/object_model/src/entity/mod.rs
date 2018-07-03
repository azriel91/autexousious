//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::character::CharacterInput;
pub use self::object_status::ObjectStatus;
pub use self::object_status_update::ObjectStatusUpdate;
pub use self::position::Position;

mod character;
mod object_status;
mod object_status_update;
mod position;
