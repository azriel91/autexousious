use std::collections::HashMap;

use amethyst::{assets::Handle, Error};
use character_model::{
    config::{CharacterDefinition, CharacterSequenceId},
    loaded::{Character, CharacterObjectWrapper},
};
use game_input_model::ControlAction;
use object_model::config::GameObjectSequence;
use sequence_model::loaded::{
    ControlTransition, ControlTransitionHold, ControlTransitionPress, ControlTransitions,
    ControlTransitionsSequence,
};

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
        let control_transitions_sequences = character_definition.object_definition.sequences.iter()
            .map(|(sequence_id, sequence)| {
                let control_transitions_sequence = sequence.object_sequence().frames.iter()
                    .map(|frame| {
                        let control_action_transitions = &frame.transitions;
                        let mut control_transitions = Vec::new();

                        macro_rules! insert_if_some {
                            ($conf_field:ident, $input_type:ident, $type_data:ident, $button:ident) => {
                                if let Some(sequence_id) = control_action_transitions.$conf_field {
                                    control_transitions.push(
                                        ControlTransition::$input_type(
                                            $type_data { action: ControlAction::$button, sequence_id }
                                        )
                                    );
                                }
                            };
                        }

                        insert_if_some!(press_defend, Press, ControlTransitionPress, Defend);
                        insert_if_some!(press_jump, Press, ControlTransitionPress, Jump);
                        insert_if_some!(press_attack, Press, ControlTransitionPress, Attack);
                        insert_if_some!(press_special, Press, ControlTransitionPress, Special);
                        insert_if_some!(hold_defend, Hold, ControlTransitionHold, Defend);
                        insert_if_some!(hold_jump, Hold, ControlTransitionHold, Jump);
                        insert_if_some!(hold_attack, Hold, ControlTransitionHold, Attack);
                        insert_if_some!(hold_special, Hold, ControlTransitionHold, Special);

                        ControlTransitions::new(control_transitions)
                    }).collect::<Vec<ControlTransitions<CharacterSequenceId>>>();
                let control_transitions_sequence = ControlTransitionsSequence::new(control_transitions_sequence);

                let control_transitions_sequence_handle = loader.load_from_data(
                    control_transitions_sequence,
                    (),
                    control_transitions_sequence_assets,
                );

                (
                    *sequence_id,
                    control_transitions_sequence_handle,
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Character::new(
            control_transitions_sequences,
            object_wrapper_handle,
        ))
    }
}
