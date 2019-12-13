use std::{convert::AsRef, default::Default, marker::PhantomData};

use amethyst::assets::{AssetStorage, Loader};
use game_input_model::{Axis, ControlAction};
use input_reaction_model::{
    config::{self, InputReactionSingle},
    loaded::{
        self, AxisTransition, FallbackTransition, InputReaction, InputReactions,
        InputReactionsSequence, InputReactionsSequenceHandle, ReactionEffect, ReactionEffectData,
    },
};
use sequence_model::{
    config::{Sequence, SequenceName, Wait},
    loaded::SequenceIdMappings,
};
use typename::TypeName;

use crate::IrsLoaderParams;

/// Loads `InputReactionsSequence` configuration into the loaded model.
#[derive(Debug)]
pub struct IrsLoader<Seq, SeqName, IRR, Frm>(PhantomData<(Seq, SeqName, IRR, Frm)>);

impl<Seq, SeqName, IRR, Frm> IrsLoader<Seq, SeqName, IRR, Frm>
where
    Seq: AsRef<Sequence<SeqName, Frm>> + AsRef<Option<config::InputReactions<SeqName, IRR>>>,
    SeqName: SequenceName,
    IRR: Clone + Default + Send + Sync + TypeName + 'static,
    Frm: AsRef<Wait> + AsRef<config::InputReactions<SeqName, IRR>>,
{
    /// Extracts an `InputReactionsSequence` from a `Sequence`.
    pub fn load(
        IrsLoaderParams {
            loader,
            input_reactions_assets,
            input_reactions_sequence_assets,
        }: &IrsLoaderParams<IRR>,
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        sequence_default: Option<&Seq>,
        seq: &Seq,
    ) -> InputReactionsSequenceHandle<InputReaction<IRR>> {
        let sequence = AsRef::<Sequence<SeqName, Frm>>::as_ref(seq);
        let input_reactions_sequence = sequence
            .frames
            .iter()
            .map(|frame| {
                let config_transitions_default = sequence_default.and_then(|sequence| {
                    AsRef::<Option<config::InputReactions<SeqName, IRR>>>::as_ref(sequence).as_ref()
                });
                Self::config_to_loaded_transitions_handle(
                    loader,
                    input_reactions_assets,
                    sequence_id_mappings,
                    config_transitions_default,
                    AsRef::<Option<config::InputReactions<SeqName, IRR>>>::as_ref(seq).as_ref(),
                    &AsRef::<config::InputReactions<SeqName, IRR>>::as_ref(frame),
                )
            })
            .collect::<Vec<loaded::InputReactionsHandle<InputReaction<IRR>>>>();

        let input_reactions_sequence = InputReactionsSequence::new(input_reactions_sequence);

        loader.load_from_data(
            input_reactions_sequence,
            (),
            input_reactions_sequence_assets,
        )
    }

    /// Maps `config::InputReactions::<InputReaction<IRR>>` to `loaded::InputReactions::<InputReaction<IRR>>`
    fn config_to_loaded_transitions_handle(
        loader: &Loader,
        input_reactions_assets: &AssetStorage<loaded::InputReactions<InputReaction<IRR>>>,
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        config_transitions_default: Option<&config::InputReactions<SeqName, IRR>>,
        config_transitions_sequence: Option<&config::InputReactions<SeqName, IRR>>,
        config_transitions_frame: &config::InputReactions<SeqName, IRR>,
    ) -> loaded::InputReactionsHandle<InputReaction<IRR>> {
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
                    use input_reaction_model::config::InputReaction::*;
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
                            loaded_transitions.push(InputReaction::<IRR>::new(
                                ReactionEffect::$mode($mode_data {
                                    action: ControlAction::$action,
                                    sequence_id: *sequence_id,
                                    events: Default::default(),
                                }),
                                IRR::default(),
                            ));
                        }
                        Single(InputReactionSingle {
                            next: sequence_name_string,
                            events,
                            requirement,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                            loaded_transitions.push(InputReaction::<IRR>::new(
                                ReactionEffect::$mode($mode_data {
                                    action: ControlAction::$action,
                                    sequence_id: *sequence_id,
                                    events: events.clone(),
                                }),
                                requirement.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |InputReactionSingle {
                                 next: sequence_name_string,
                                 events,
                                 requirement,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                                InputReaction::<IRR>::new(
                                    ReactionEffect::$mode($mode_data {
                                        action: ControlAction::$action,
                                        sequence_id: *sequence_id,
                                        events: events.clone(),
                                    }),
                                    requirement.clone(),
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
                    use input_reaction_model::config::InputReaction::*;
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
                            loaded_transitions.push(InputReaction::<IRR>::new(
                                ReactionEffect::$mode(AxisTransition {
                                    axis: Axis::$axis,
                                    sequence_id: *sequence_id,
                                    events: Default::default(),
                                }),
                                IRR::default(),
                            ));
                        }
                        Single(InputReactionSingle {
                            next: sequence_name_string,
                            events,
                            requirement,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                            loaded_transitions.push(InputReaction::<IRR>::new(
                                ReactionEffect::$mode(AxisTransition {
                                    axis: Axis::$axis,
                                    sequence_id: *sequence_id,
                                    events: events.clone(),
                                }),
                                requirement.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |InputReactionSingle {
                                 next: sequence_name_string,
                                 events,
                                 requirement,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                                InputReaction::<IRR>::new(
                                    ReactionEffect::$mode(AxisTransition {
                                        axis: Axis::$axis,
                                        sequence_id: *sequence_id,
                                        events: events.clone(),
                                    }),
                                    requirement.clone(),
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
                    use input_reaction_model::config::InputReaction::*;
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
                            loaded_transitions.push(InputReaction::<IRR>::new(
                                ReactionEffect::$mode(FallbackTransition {
                                    sequence_id: *sequence_id,
                                    events: Default::default(),
                                }),
                                IRR::default(),
                            ));
                        }
                        Single(InputReactionSingle {
                            next: sequence_name_string,
                            events,
                            requirement,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                            loaded_transitions.push(InputReaction::<IRR>::new(
                                ReactionEffect::$mode(FallbackTransition {
                                    sequence_id: *sequence_id,
                                    events: events.clone(),
                                }),
                                requirement.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |InputReactionSingle {
                                 next: sequence_name_string,
                                 events,
                                 requirement,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_name_string).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name_string
                                    )
                                });
                                InputReaction::<IRR>::new(
                                    ReactionEffect::$mode(FallbackTransition {
                                        sequence_id: *sequence_id,
                                        events: events.clone(),
                                    }),
                                    requirement.clone(),
                                )
                            },
                        )),
                    }
                }
            };
        }

        push_transitions!(press_defend, ActionPress, ReactionEffectData, Defend);
        push_transitions!(press_jump, ActionPress, ReactionEffectData, Jump);
        push_transitions!(press_attack, ActionPress, ReactionEffectData, Attack);
        push_transitions!(press_special, ActionPress, ReactionEffectData, Special);
        push_transitions!(release_defend, ActionRelease, ReactionEffectData, Defend);
        push_transitions!(release_jump, ActionRelease, ReactionEffectData, Jump);
        push_transitions!(release_attack, ActionRelease, ReactionEffectData, Attack);
        push_transitions!(release_special, ActionRelease, ReactionEffectData, Special);
        // It is a requirement that we push the `Hold` transitions last, to ensure the `Press` and
        // `Release` transitions get higher priority.
        push_transitions!(hold_defend, ActionHold, ReactionEffectData, Defend);
        push_transitions!(hold_jump, ActionHold, ReactionEffectData, Jump);
        push_transitions!(hold_attack, ActionHold, ReactionEffectData, Attack);
        push_transitions!(hold_special, ActionHold, ReactionEffectData, Special);

        // Axes transitions.
        push_axis_transition!(press_x, AxisPress, X);
        push_axis_transition!(press_z, AxisPress, Z);
        push_axis_transition!(release_x, AxisRelease, X);
        push_axis_transition!(release_z, AxisRelease, Z);
        push_axis_transition!(hold_x, AxisHold, X);
        push_axis_transition!(hold_z, AxisHold, Z);

        // Fallback transition.
        push_fallback_transition!(fallback, Fallback);

        let character_input_reactions = InputReactions::new(loaded_transitions);

        loader.load_from_data(character_input_reactions, (), input_reactions_assets)
    }
}
