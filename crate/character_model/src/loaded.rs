//! Contains the types that represent processed configuration.
//!
//! This differs from the plain configuration types as they would have been processed into the form
//! that will be used in game.

pub use self::{
    asset_character_definition_handle::AssetCharacterDefinitionHandle,
    character_hit_transitions::CharacterHitTransitions,
    character_input_reaction::CharacterInputReaction,
    character_input_reactions::{CharacterInputReactions, CharacterInputReactionsHandle},
    character_irs::{CharacterIrs, CharacterIrsHandle},
    character_irs_handles::CharacterIrsHandles,
};

mod asset_character_definition_handle;
mod character_hit_transitions;
mod character_input_reaction;
mod character_input_reactions;
mod character_irs;
mod character_irs_handles;
