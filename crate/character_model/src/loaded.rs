//! Contains the types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::{
    asset_character_cts_handles::AssetCharacterCtsHandles,
    asset_character_definition_handle::AssetCharacterDefinitionHandle,
    character_control_transition::CharacterControlTransition,
    character_control_transitions::{
        CharacterControlTransitions, CharacterControlTransitionsHandle,
    },
    character_cts::{CharacterCts, CharacterCtsHandle},
    character_cts_handles::CharacterCtsHandles,
    character_hit_transitions::CharacterHitTransitions,
};

mod asset_character_cts_handles;
mod asset_character_definition_handle;
mod character_control_transition;
mod character_control_transitions;
mod character_cts;
mod character_cts_handles;
mod character_hit_transitions;
