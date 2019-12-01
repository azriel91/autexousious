use amethyst::assets::{AssetStorage, Loader};
use character_model::{
    config::{self, CharacterSequence, CharacterSequenceName},
    loaded::{self, CharacterControlTransition, CharacterIrs, CharacterIrsHandle},
};
use game_input_model::{Axis, ControlAction};
use object_model::config::GameObjectSequence;
use sequence_model::{
    config::ControlTransitionSingle,
    loaded::{
        ActionHold, ActionPress, ActionRelease, AxisTransition, FallbackTransition, InputReaction,
        InputReactions, SequenceIdMappings,
    },
};

use crate::IrsLoaderParams;

/// Loads `InputReactionsSequence` configuration into the loaded model.
#[derive(Debug)]
pub enum IrsLoader {}

impl IrsLoader {
    /// Extracts a `CharacterIrs` from a `CharacterSequence`.
    pub fn load(
        IrsLoaderParams {
            loader,
            character_input_reactions_assets,
            character_irs_assets,
        }: &IrsLoaderParams,
        sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>,
        sequence_default: Option<&CharacterSequence>,
        sequence: &CharacterSequence,
    ) -> CharacterIrsHandle {
        let irs = sequence
            .object_sequence()
            .frames
            .iter()
            .map(|frame| {
                let config_transitions_default =
                    sequence_default.and_then(|sequence| sequence.input_reactions.as_ref());
                Self::config_to_loaded_transitions_handle(
                    loader,
                    character_input_reactions_assets,
                    sequence_id_mappings,
                    config_transitions_default,
                    sequence.input_reactions.as_ref(),
                    &frame.input_reactions,
                )
            })
            .collect::<Vec<loaded::CharacterInputReactionsHandle>>();

        let character_irs = CharacterIrs::new(irs);

        loader.load_from_data(character_irs, (), character_irs_assets)
    }

    /// Maps `config::CharacterInputReactions` to `loaded::CharacterInputReactions`
    fn config_to_loaded_transitions_handle(
        loader: &Loader,
        character_input_reactions_assets: &AssetStorage<loaded::CharacterInputReactions>,
        sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>,
        config_transitions_default: Option<&config::CharacterInputReactions>,
        config_transitions_sequence: Option<&config::CharacterInputReactions>,
        config_transitions_frame: &config::CharacterInputReactions,
    ) -> loaded::CharacterInputReactionsHandle {
        let mut loaded_transitions = Vec::new();

        macro_rules! push_transitions {
            ($mode_action:ident, $mode:ident, $mode_data:ident, $action:ident) => {
                let mode_action = config_transitions_frame.$mode_action.as_ref().or_else(|| {
                    // We want to make sure that, if `config_transitions_sequence.is_some()`, but
                    // the transition inside is `None`, we still fallback to `None`. This allows
                    // a sequence transition `None` value to override the default transition.
                    config_transitions_sequence
                        .or(config_transitions_default)
                        .and_then(|config_transitions_fallback| {
                            config_transitions_fallback.$mode_action.as_ref()
                        })
                });
                if let Some(config_control_transition) = &mode_action {
                    use sequence_model::config::InputReaction::*;
                    match config_control_transition {
                        SequenceNameString(sequence_name) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                InputReaction::$mode($mode_data {
                                    action: ControlAction::$action,
                                    sequence_id: *sequence_id,
                                }),
                                vec![],
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_name_string,
                            requirements: control_transition_requirements,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                InputReaction::$mode($mode_data {
                                    action: ControlAction::$action,
                                    sequence_id: *sequence_id,
                                }),
                                control_transition_requirements.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_name_string,
                                 requirements: control_transition_requirements,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                                CharacterControlTransition::new(
                                    InputReaction::$mode($mode_data {
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

        macro_rules! push_axis_transition {
            ($mode_action:ident, $mode:ident, $axis:ident) => {
                let mode_action = config_transitions_frame.$mode_action.as_ref().or_else(|| {
                    // We want to make sure that, if `config_transitions_sequence.is_some()`, but
                    // the transition inside is `None`, we still fallback to `None`. This allows
                    // a sequence transition `None` value to override the default transition.
                    config_transitions_sequence
                        .or(config_transitions_default)
                        .and_then(|config_transitions_fallback| {
                            config_transitions_fallback.$mode_action.as_ref()
                        })
                });
                if let Some(config_control_transition) = &mode_action {
                    use sequence_model::config::InputReaction::*;
                    match config_control_transition {
                        SequenceNameString(sequence_name) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                InputReaction::$mode(AxisTransition {
                                    axis: Axis::$axis,
                                    sequence_id: *sequence_id,
                                }),
                                vec![],
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_name_string,
                            requirements: control_transition_requirements,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                InputReaction::$mode(AxisTransition {
                                    axis: Axis::$axis,
                                    sequence_id: *sequence_id,
                                }),
                                control_transition_requirements.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_name_string,
                                 requirements: control_transition_requirements,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                                CharacterControlTransition::new(
                                    InputReaction::$mode(AxisTransition {
                                        axis: Axis::$axis,
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

        macro_rules! push_fallback_transition {
            ($mode_action:ident, $mode:ident) => {
                let mode_action = config_transitions_frame.$mode_action.as_ref().or_else(|| {
                    // We want to make sure that, if `config_transitions_sequence.is_some()`, but
                    // the transition inside is `None`, we still fallback to `None`. This allows
                    // a sequence transition `None` value to override the default transition.
                    config_transitions_sequence
                        .or(config_transitions_default)
                        .and_then(|config_transitions_fallback| {
                            config_transitions_fallback.$mode_action.as_ref()
                        })
                });
                if let Some(config_control_transition) = &mode_action {
                    use sequence_model::config::InputReaction::*;
                    match config_control_transition {
                        SequenceNameString(sequence_name) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                InputReaction::$mode(FallbackTransition {
                                    sequence_id: *sequence_id,
                                }),
                                vec![],
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_name_string,
                            requirements: control_transition_requirements,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                InputReaction::$mode(FallbackTransition {
                                    sequence_id: *sequence_id,
                                }),
                                control_transition_requirements.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_name_string,
                                 requirements: control_transition_requirements,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                                CharacterControlTransition::new(
                                    InputReaction::$mode(FallbackTransition {
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

        push_transitions!(press_defend, ActionPress, ActionPress, Defend);
        push_transitions!(press_jump, ActionPress, ActionPress, Jump);
        push_transitions!(press_attack, ActionPress, ActionPress, Attack);
        push_transitions!(press_special, ActionPress, ActionPress, Special);
        push_transitions!(release_defend, ActionRelease, ActionRelease, Defend);
        push_transitions!(release_jump, ActionRelease, ActionRelease, Jump);
        push_transitions!(release_attack, ActionRelease, ActionRelease, Attack);
        push_transitions!(release_special, ActionRelease, ActionRelease, Special);
        // It is a requirement that we push the `Hold` transitions last, to ensure the `Press` and
        // `Release` transitions get higher priority.
        push_transitions!(hold_defend, ActionHold, ActionHold, Defend);
        push_transitions!(hold_jump, ActionHold, ActionHold, Jump);
        push_transitions!(hold_attack, ActionHold, ActionHold, Attack);
        push_transitions!(hold_special, ActionHold, ActionHold, Special);

        // Axes transitions.
        push_axis_transition!(press_x, AxisPress, X);
        push_axis_transition!(press_z, AxisPress, Z);
        push_axis_transition!(release_x, AxisRelease, X);
        push_axis_transition!(release_z, AxisRelease, Z);
        push_axis_transition!(hold_x, AxisHold, X);
        push_axis_transition!(hold_z, AxisHold, Z);

        // Fallback transition.
        push_fallback_transition!(fallback, Fallback);

        let character_input_reactions =
            loaded::CharacterInputReactions::new(InputReactions::new(loaded_transitions));

        loader.load_from_data(
            character_input_reactions,
            (),
            character_input_reactions_assets,
        )
    }
}
