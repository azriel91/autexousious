//! Contains the types that represent the configuration on disk.

pub use self::{
    character_definition::{CharacterDefinition, CharacterDefinitionHandle},
    character_frame::CharacterFrame,
    character_input_reactions::CharacterInputReactions,
    character_sequence::CharacterSequence,
    character_sequence_name::CharacterSequenceName,
    character_sequence_name_string::CharacterSequenceNameString,
    input_reaction_requirement::InputReactionRequirement,
    input_reaction_requirement_params::InputReactionRequirementParams,
};

mod character_definition;
mod character_frame;
mod character_input_reactions;
mod character_sequence;
mod character_sequence_name;
mod character_sequence_name_string;
mod input_reaction_requirement;
mod input_reaction_requirement_params;
