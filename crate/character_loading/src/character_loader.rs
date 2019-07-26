use std::collections::HashMap;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    Error,
};
use character_model::{
    config::{self, CharacterDefinition, CharacterSequence},
    loaded::{
        self, Character, CharacterControlTransition, CharacterControlTransitionsSequence,
        CharacterControlTransitionsSequenceHandle, CharacterObjectWrapper,
    },
};
use game_input_model::ControlAction;
use object_model::config::GameObjectSequence;
use sequence_model::{
    config::ControlTransitionSingle,
    loaded::{
        ControlTransition, ControlTransitionHold, ControlTransitionPress, ControlTransitionRelease,
        ControlTransitions,
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
        character_loader_params: CharacterLoaderParams,
        character_definition: &CharacterDefinition,
        object_wrapper_handle: Handle<CharacterObjectWrapper>,
    ) -> Result<Character, Error> {
        let control_transitions_sequence_handles = character_definition
            .object_definition
            .sequences
            .iter()
            .map(|(sequence_id, sequence)| {
                (
                    *sequence_id,
                    Self::control_transitions_sequence_handle(&character_loader_params, sequence),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Character::new(
            control_transitions_sequence_handles,
            object_wrapper_handle,
        ))
    }

    /// Extracts a `CharacterControlTransitionsSequence` from a `CharacterSequence`.
    fn control_transitions_sequence_handle(
        CharacterLoaderParams {
            loader,
            character_control_transitions_assets,
            character_control_transitions_sequence_assets,
        }: &CharacterLoaderParams,
        sequence: &CharacterSequence,
    ) -> CharacterControlTransitionsSequenceHandle {
        let control_transitions_sequence = sequence
            .object_sequence()
            .frames
            .iter()
            .map(|frame| {
                Self::config_to_loaded_transitions_handle(
                    loader,
                    character_control_transitions_assets,
                    &frame.transitions,
                )
            })
            .collect::<Vec<loaded::CharacterControlTransitionsHandle>>();

        let character_control_transitions_sequence =
            CharacterControlTransitionsSequence::new(control_transitions_sequence);

        loader.load_from_data(
            character_control_transitions_sequence,
            (),
            character_control_transitions_sequence_assets,
        )
    }

    /// Maps `config::CharacterControlTransitions` to `loaded::CharacterControlTransitions`
    fn config_to_loaded_transitions_handle(
        loader: &Loader,
        character_control_transitions_assets: &AssetStorage<loaded::CharacterControlTransitions>,
        config_transitions: &config::CharacterControlTransitions,
    ) -> loaded::CharacterControlTransitionsHandle {
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
                                vec![],
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_id,
                            requirements: control_transition_requirements,
                        }) => loaded_transitions.push(CharacterControlTransition::new(
                            ControlTransition::$mode($mode_data {
                                action: ControlAction::$action,
                                sequence_id: *sequence_id,
                            }),
                            control_transition_requirements.clone(),
                        )),
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_id,
                                 requirements: control_transition_requirements,
                             }| {
                                CharacterControlTransition::new(
                                    ControlTransition::$mode($mode_data {
                                        action: ControlAction::$action,
                                        sequence_id: *sequence_id,
                                    }),
                                    control_transition_requirements.clone(),
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
        push_transitions!(release_defend, Release, ControlTransitionRelease, Defend);
        push_transitions!(release_jump, Release, ControlTransitionRelease, Jump);
        push_transitions!(release_attack, Release, ControlTransitionRelease, Attack);
        push_transitions!(release_special, Release, ControlTransitionRelease, Special);
        // It is a requirement that we push the `Hold` transitions last, to ensure the `Press` and
        // `Release` transitions get higher priority.
        push_transitions!(hold_defend, Hold, ControlTransitionHold, Defend);
        push_transitions!(hold_jump, Hold, ControlTransitionHold, Jump);
        push_transitions!(hold_attack, Hold, ControlTransitionHold, Attack);
        push_transitions!(hold_special, Hold, ControlTransitionHold, Special);

        let character_control_transitions =
            loaded::CharacterControlTransitions::new(ControlTransitions::new(loaded_transitions));

        loader.load_from_data(
            character_control_transitions,
            (),
            character_control_transitions_assets,
        )
    }
}
