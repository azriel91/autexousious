//! Contains the types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::animation::AnimatedComponentAnimation;
pub use self::character::{Character, CharacterHandle};
pub use self::object::Object;

mod animation;
mod character;
mod object;
