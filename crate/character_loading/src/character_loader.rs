use std::collections::HashMap;

use amethyst::{assets::Handle, Error};
use character_model::{
    config::{self, CharacterDefinition, CharacterSequence, ControlTransitionRequirement},
    loaded::{
        self, Character, CharacterControlTransition, CharacterControlTransitionsSequence,
        CharacterObjectWrapper,
    },
};
use game_input_model::ControlAction;
use object_model::config::GameObjectSequence;
use sequence_model::{
    config::ControlTransitionSingle,
    loaded::{
        ControlTransition, ControlTransitionHold, ControlTransitionPress, ControlTransitionRelease,
    },
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
            character_control_transitions_sequence_assets,
        }: CharacterLoaderParams,
        character_definition: &CharacterDefinition,
        object_wrapper_handle: Handle<CharacterObjectWrapper>,
    ) -> Result<Character, Error> {
        let control_transitions_sequence_handles = character_definition
            .object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (*sequence_id, Self::control_transitions_sequence(sequence))
            })
            .map(|(sequence_id, character_control_transitions_sequence)| {
                let handle = loader.load_from_data(
                    character_control_transitions_sequence,
                    (),
                    character_control_transitions_sequence_assets,
                );

                (sequence_id, handle)
            })
            .collect::<HashMap<_, _>>();

        Ok(Character::new(
            control_transitions_sequence_handles,
            object_wrapper_handle,
        ))
    }

    /// Extracts a `CharacterControlTransitionsSequence` from a `CharacterSequence`.
    fn control_transitions_sequence(
        sequence: &CharacterSequence,
    ) -> CharacterControlTransitionsSequence {
        let control_transitions_sequence = sequence
            .object_sequence()
            .frames
            .iter()
            .map(|frame| Self::config_to_loaded_transitions(&frame.transitions))
            .collect::<Vec<loaded::CharacterControlTransitions>>();

        CharacterControlTransitionsSequence::new(control_transitions_sequence)
    }

    /// Maps `config::CharacterControlTransitions` to `loaded::CharacterControlTransitions`
    fn config_to_loaded_transitions(
        config_transitions: &config::CharacterControlTransitions,
    ) -> loaded::CharacterControlTransitions {
        let mut loaded_transitions = Vec::new();

        macro_rules! push_transitions {
            ($mode_action:ident, $mode:ident, $mode_data:ident, $action:ident) => {
                if let Some(config_control_transition) = &config_transitions.$mode_action {
                    use sequence_model::config::ControlTransition::*;
                    match config_control_transition {
                        SequenceId(sequence_id) => {
                            loaded_transitions.push(CharacterControlTransition::new(
                                ControlTransition::$mode($mode_data {
                                    action: ControlAction::$action,
                                    sequence_id: *sequence_id,
                                }),
                                None,
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_id,
                            extra: control_transition_requirement,
                        }) => loaded_transitions.push(CharacterControlTransition::new(
                            ControlTransition::$mode($mode_data {
                                action: ControlAction::$action,
                                sequence_id: *sequence_id,
                            }),
                            Self::requirement_opt(*control_transition_requirement),
                        )),
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_id,
                                 extra: control_transition_requirement,
                             }| {
                                CharacterControlTransition::new(
                                    ControlTransition::$mode($mode_data {
                                        action: ControlAction::$action,
                                        sequence_id: *sequence_id,
                                    }),
                                    Self::requirement_opt(*control_transition_requirement),
                                )
                            },
                        )),
                    }
                }
            };
        }

        push_transitions!(press_defend, Press, ControlTransitionPress, Defend);
        push_transitions!(press_jump, Press, ControlTransitionPress, Jump);
        push_transitions!(press_attack, Press, ControlTransitionPress, Attack);
        push_transitions!(press_special, Press, ControlTransitionPress, Special);
        push_transitions!(hold_defend, Hold, ControlTransitionHold, Defend);
        push_transitions!(hold_jump, Hold, ControlTransitionHold, Jump);
        push_transitions!(hold_attack, Hold, ControlTransitionHold, Attack);
        push_transitions!(hold_special, Hold, ControlTransitionHold, Special);
        push_transitions!(release_defend, Release, ControlTransitionRelease, Defend);
        push_transitions!(release_jump, Release, ControlTransitionRelease, Jump);
        push_transitions!(release_attack, Release, ControlTransitionRelease, Attack);
        push_transitions!(release_special, Release, ControlTransitionRelease, Special);

        loaded::CharacterControlTransitions::new(loaded_transitions)
    }

    #[inline]
    fn requirement_opt(
        requirement: ControlTransitionRequirement,
    ) -> Option<ControlTransitionRequirement> {
        if requirement.is_blank() {
            None
        } else {
            Some(requirement)
        }
    }
}
