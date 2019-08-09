use std::collections::HashMap;

use amethyst::{assets::Handle, Error};
use character_model::{
    config::CharacterDefinition,
    loaded::{Character, CharacterObjectWrapper},
};
use lazy_static::lazy_static;

use crate::{CharacterLoaderParams, ControlTransitionsSequenceLoader};

lazy_static! {
    /// Default `CharacterDefinition` with control transitions.
    pub static ref CHARACTER_TRANSITIONS_DEFAULT: CharacterDefinition = {
        let definition_toml = include_str!("character_transitions_default.toml");
        toml::from_str::<CharacterDefinition>(definition_toml)
            .expect("Failed to deserialize `character_transitions_default.toml`.")
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
                    sequence_default,
                    sequence,
                );
                (*sequence_id, control_transitions_sequence_handle)
            })
            .collect::<HashMap<_, _>>();

        Ok(Character::new(
            control_transitions_sequence_handles,
            object_wrapper_handle,
        ))
    }
}
