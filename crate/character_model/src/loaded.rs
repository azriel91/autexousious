//! Contains the types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::{
    character::{Character, CharacterHandle, CharacterObjectWrapper},
    character_control_transition::CharacterControlTransition,
    character_control_transitions::{
        CharacterControlTransitions, CharacterControlTransitionsHandle,
    },
    character_cts::{CharacterCts, CharacterCtsHandle},
    character_hit_transitions::CharacterHitTransitions,
};

mod character;
mod character_control_transition;
mod character_control_transitions;
mod character_cts;
mod character_hit_transitions;
