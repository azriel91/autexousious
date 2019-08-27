use amethyst::{assets::Handle, Error};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterControlTransitionsSequenceHandle, CharacterObjectWrapper},
};
use lazy_static::lazy_static;
use sequence_model::loaded::{SequenceId, SequenceIdMappings};

use crate::{CharacterLoaderParams, ControlTransitionsSequenceLoader};

lazy_static! {
    /// Default `CharacterDefinition` with control transitions.
    pub static ref CHARACTER_TRANSITIONS_DEFAULT: CharacterDefinition = {
        let definition_yaml = include_str!("character_transitions_default.yaml");
        serde_yaml::from_str::<CharacterDefinition>(definition_yaml)
            .expect("Failed to deserialize `character_transitions_default.yaml`.")
    };
}

/// Loads assets specified by character configuration into the loaded character model.
#[derive(Debug)]
pub enum CharacterLoader {}

impl CharacterLoader {
    /// Returns the loaded `Character`.
    ///
    /// # Parameters
    ///
    /// * `character_loader_params`: Parameters needed to load the `Character`.
    /// * `character_definition`: Character definition asset.
    /// * `object_wrapper_handle`: Handle to the loaded `Object` for this character.
    pub fn load(
        character_loader_params: CharacterLoaderParams,
        character_definition: &CharacterDefinition,
        object_wrapper_handle: Handle<CharacterObjectWrapper>,
    ) -> Result<Character, Error> {
        // Calculate the indices of each sequence ID.
        //
        // TODO: Extract this out to a separate loading phase, as other objects may reference this
        // TODO: object's sequences.
        let capacity = character_definition.object_definition.sequences.len();
        let sequence_id_mappings = character_definition
            .object_definition
            .sequences
            .keys()
            .enumerate()
            .map(|(index, sequence_name_string)| {
                (SequenceId::new(index), sequence_name_string.clone())
            })
            .fold(
                SequenceIdMappings::with_capacity(capacity),
                |mut sequence_id_mappings, (sequence_id, sequence_name_string)| {
                    sequence_id_mappings.insert(sequence_name_string, sequence_id);
                    sequence_id_mappings
                },
            );

        let control_transitions_sequence_handles = character_definition
            .object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                let sequence_default = CHARACTER_TRANSITIONS_DEFAULT
                    .object_definition
                    .sequences
                    .get(sequence_id);
                let control_transitions_sequence_handle = ControlTransitionsSequenceLoader::load(
                    &character_loader_params.control_transitions_sequence_loader_params,
                    &sequence_id_mappings,
                    sequence_default,
                    sequence,
                );
                control_transitions_sequence_handle
            })
            .collect::<Vec<CharacterControlTransitionsSequenceHandle>>();

        Ok(Character::new(
            control_transitions_sequence_handles,
            sequence_id_mappings,
            object_wrapper_handle,
        ))
    }
}
