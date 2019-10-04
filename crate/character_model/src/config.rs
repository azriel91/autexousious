//! Contains the types that represent the configuration on disk.

pub use self::{
    character_control_transitions::CharacterControlTransitions,
    character_definition::{CharacterDefinition, CharacterDefinitionHandle},
    character_frame::CharacterFrame,
    character_sequence::CharacterSequence,
    character_sequence_name::CharacterSequenceName,
    character_sequence_name_string::CharacterSequenceNameString,
    control_transition_requirement::ControlTransitionRequirement,
    control_transition_requirement_params::ControlTransitionRequirementParams,
};

mod character_control_transitions;
mod character_definition;
mod character_frame;
mod character_sequence;
mod character_sequence_name;
mod character_sequence_name_string;
mod control_transition_requirement;
mod control_transition_requirement_params;
