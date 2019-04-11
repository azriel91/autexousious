use std::collections::HashMap;

use amethyst::{assets::Handle, Error};
use character_model::{
    config::{CharacterDefinition, CharacterSequenceId},
    loaded::{Character, CharacterObjectWrapper},
};
use object_model::config::GameObjectSequence;
use sequence_model::loaded::{ControlTransition, ControlTransitions, ControlTransitionsSequence};

use crate::CharacterLoaderParams;

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
        CharacterLoaderParams {
            loader,
            control_transitions_sequence_assets,
        }: CharacterLoaderParams,
        character_definition: &CharacterDefinition,
        object_wrapper_handle: Handle<CharacterObjectWrapper>,
    ) -> Result<Character, Error> {
        let control_transitions_sequences = character_definition
            .object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                let control_transitions_sequence = sequence
                    .object_sequence()
                    .frames
                    .iter()
                    .map(|frame| {
                        let control_transitions = frame
                            .transitions
                            .iter()
                            .map(|character_control_transition| {
                                ControlTransition::from(
                                    character_control_transition.control_transition,
                                )
                            })
                            .collect::<Vec<ControlTransition<CharacterSequenceId>>>();
                        ControlTransitions::new(control_transitions)
                    })
                    .collect::<Vec<ControlTransitions<CharacterSequenceId>>>();
                let control_transitions_sequence =
                    ControlTransitionsSequence::new(control_transitions_sequence);

                let control_transitions_sequence_handle = loader.load_from_data(
                    control_transitions_sequence,
                    (),
                    control_transitions_sequence_assets,
                );

                (*sequence_id, control_transitions_sequence_handle)
            })
            .collect::<HashMap<_, _>>();

        Ok(Character::new(
            control_transitions_sequences,
            object_wrapper_handle,
        ))
    }
}
