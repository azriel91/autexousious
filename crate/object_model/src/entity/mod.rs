//! Contains the types for object entities.
//!
//! This differs from the `loaded` types as these may contain mutable state that are specific to an
//! entity.

pub use self::character::CharacterInput;
pub use self::object_status::ObjectStatus;

mod character;
mod object_status;
