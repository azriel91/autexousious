use std::{convert::AsRef, default::Default, marker::PhantomData};

use amethyst::assets::{AssetStorage, Loader};
use game_input_model::config::{Axis, ControlAction};
use input_reaction_model::{
    config::{self, InputReactionAppEvents, InputReactionSingle},
    loaded::{
        self, AxisTransition, FallbackTransition, InputReaction, InputReactions,
        InputReactionsSequence, InputReactionsSequenceHandle, ReactionEffect, ReactionEffectButton,
        ReactionEffectData,
    },
};
use sequence_model::{
    config::{Sequence, SequenceName, SequenceNameString, Wait},
    loaded::{SequenceId, SequenceIdMappings},
};

use crate::IrsLoaderParams;

/// Loads `InputReactionsSequence` configuration into the loaded model.
#[derive(Debug)]
pub struct IrsLoader<Seq, SeqName, IRR, Frm>(PhantomData<(Seq, SeqName, IRR, Frm)>);

impl<Seq, SeqName, IRR, Frm> IrsLoader<Seq, SeqName, IRR, Frm>
where
    Seq: AsRef<Sequence<SeqName, Frm>> + AsRef<Option<config::InputReactions<SeqName, IRR>>>,
    SeqName: SequenceName,
    IRR: Clone + Default + Send + Sync + 'static,
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
                let input_reactions_default = sequence_default.and_then(|sequence| {
                    AsRef::<Option<config::InputReactions<SeqName, IRR>>>::as_ref(sequence).as_ref()
                });
                Self::input_reactions_loaded_handle(
                    loader,
                    input_reactions_assets,
                    sequence_id_mappings,
                    input_reactions_default,
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
    fn input_reactions_loaded_handle(
        loader: &Loader,
        input_reactions_assets: &AssetStorage<loaded::InputReactions<InputReaction<IRR>>>,
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        input_reactions_default: Option<&config::InputReactions<SeqName, IRR>>,
        input_reactions_sequence: Option<&config::InputReactions<SeqName, IRR>>,
        input_reactions_frame: &config::InputReactions<SeqName, IRR>,
    ) -> loaded::InputReactionsHandle<InputReaction<IRR>> {
        let mut input_reactions_loaded = Vec::new();

        macro_rules! push_action_reactions {
            ($mode_action:ident, $mode:ident, $action:ident) => {
                let mode_action = input_reactions_frame.$mode_action.as_ref().or_else(|| {
                    // We want to make sure that, if `input_reactions_sequence.is_some()`, but
                    // the transition inside is `None`, we still fallback to `None`. This allows
                    // a sequence transition `None` value to override the default transition.
                    input_reactions_sequence
                        .or(input_reactions_default)
                        .and_then(|input_reactions_fallback| {
                            input_reactions_fallback.$mode_action.as_ref()
                        })
                });
                Self::load_input_reactions(
                    sequence_id_mappings,
                    &mut input_reactions_loaded,
                    mode_action,
                    |sequence_id, events, requirement| {
                        InputReaction::<IRR>::new(
                            ReactionEffect::$mode(ReactionEffectData {
                                action: ControlAction::$action,
                                sequence_id,
                                events,
                            }),
                            requirement,
                        )
                    },
                );
            };
        }

        macro_rules! push_axis_reactions {
            ($mode_action:ident, $mode:ident, $axis:ident) => {
                let mode_action = input_reactions_frame.$mode_action.as_ref().or_else(|| {
                    // We want to make sure that, if `input_reactions_sequence.is_some()`, but
                    // the transition inside is `None`, we still fallback to `None`. This allows
                    // a sequence transition `None` value to override the default transition.
                    input_reactions_sequence
                        .or(input_reactions_default)
                        .and_then(|input_reactions_fallback| {
                            input_reactions_fallback.$mode_action.as_ref()
                        })
                });
                Self::load_input_reactions(
                    sequence_id_mappings,
                    &mut input_reactions_loaded,
                    mode_action,
                    |sequence_id, events, requirement| {
                        InputReaction::<IRR>::new(
                            ReactionEffect::$mode(AxisTransition {
                                axis: Axis::$axis,
                                sequence_id,
                                events,
                            }),
                            requirement,
                        )
                    },
                );
            };
        }

        push_action_reactions!(press_defend, ActionPress, Defend);
        push_action_reactions!(press_jump, ActionPress, Jump);
        push_action_reactions!(press_attack, ActionPress, Attack);
        push_action_reactions!(press_special, ActionPress, Special);
        push_action_reactions!(release_defend, ActionRelease, Defend);
        push_action_reactions!(release_jump, ActionRelease, Jump);
        push_action_reactions!(release_attack, ActionRelease, Attack);
        push_action_reactions!(release_special, ActionRelease, Special);
        // It is a requirement that we push the `Hold` transitions last, to ensure the `Press` and
        // `Release` transitions get higher priority.
        push_action_reactions!(hold_defend, ActionHold, Defend);
        push_action_reactions!(hold_jump, ActionHold, Jump);
        push_action_reactions!(hold_attack, ActionHold, Attack);
        push_action_reactions!(hold_special, ActionHold, Special);

        // Axes transitions.
        push_axis_reactions!(press_x, AxisPress, X);
        push_axis_reactions!(press_z, AxisPress, Z);
        push_axis_reactions!(release_x, AxisRelease, X);
        push_axis_reactions!(release_z, AxisRelease, Z);
        push_axis_reactions!(hold_x, AxisHold, X);
        push_axis_reactions!(hold_z, AxisHold, Z);

        // Fallback transition.
        Self::push_fallback_reaction(
            input_reactions_default,
            input_reactions_sequence,
            input_reactions_frame,
            sequence_id_mappings,
            &mut input_reactions_loaded,
        );

        // Device button press.
        Self::push_button_reaction(
            input_reactions_default,
            input_reactions_sequence,
            input_reactions_frame,
            sequence_id_mappings,
            &mut input_reactions_loaded,
        );

        let character_input_reactions = InputReactions::new(input_reactions_loaded);

        loader.load_from_data(character_input_reactions, (), input_reactions_assets)
    }

    fn push_fallback_reaction(
        input_reactions_default: Option<&config::InputReactions<SeqName, IRR>>,
        input_reactions_sequence: Option<&config::InputReactions<SeqName, IRR>>,
        input_reactions_frame: &config::InputReactions<SeqName, IRR>,
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        input_reactions_loaded: &mut Vec<InputReaction<IRR>>,
    ) {
        let mode_action = input_reactions_frame.fallback.as_ref().or_else(|| {
            // We want to make sure that, if `input_reactions_sequence.is_some()`, but
            // the transition inside is `None`, we still fallback to `None`. This allows
            // a sequence transition `None` value to override the default transition.
            input_reactions_sequence
                .or(input_reactions_default)
                .and_then(|input_reactions_fallback| input_reactions_fallback.fallback.as_ref())
        });
        Self::load_input_reactions(
            sequence_id_mappings,
            input_reactions_loaded,
            mode_action,
            |sequence_id, events, requirement| {
                InputReaction::<IRR>::new(
                    ReactionEffect::Fallback(FallbackTransition {
                        sequence_id,
                        events,
                    }),
                    requirement,
                )
            },
        );
    }

    fn push_button_reaction(
        input_reactions_default: Option<&config::InputReactions<SeqName, IRR>>,
        input_reactions_sequence: Option<&config::InputReactions<SeqName, IRR>>,
        input_reactions_frame: &config::InputReactions<SeqName, IRR>,
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        input_reactions_loaded: &mut Vec<InputReaction<IRR>>,
    ) {
        let button_input_reaction_single =
            input_reactions_frame.press_button.as_ref().or_else(|| {
                // We want to make sure that, if `input_reactions_sequence.is_some()`, but
                // the transition inside is `None`, we still fallback to `None`. This allows
                // a sequence transition `None` value to override the default transition.
                input_reactions_sequence
                    .or(input_reactions_default)
                    .and_then(|input_reactions_fallback| {
                        input_reactions_fallback.press_button.as_ref()
                    })
            });
        if let Some(button_input_reaction_single) = button_input_reaction_single {
            let button = button_input_reaction_single.button;

            Self::load_input_reactions(
                sequence_id_mappings,
                input_reactions_loaded,
                Some(button_input_reaction_single),
                |sequence_id, events, requirement| {
                    InputReaction::<IRR>::new(
                        ReactionEffect::ButtonPress(ReactionEffectButton {
                            button,
                            sequence_id,
                            events,
                        }),
                        requirement,
                    )
                },
            );
        }
    }

    fn load_input_reactions<IR, F>(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        input_reactions_loaded: &mut Vec<InputReaction<IRR>>,
        mode_action: Option<&IR>,
        fn_input_reaction: F,
    ) where
        IR: AsRef<config::InputReaction<SeqName, IRR>>,
        F: Fn(SequenceId, InputReactionAppEvents, IRR) -> InputReaction<IRR>,
    {
        let mode_action = mode_action.map(AsRef::<config::InputReaction<SeqName, IRR>>::as_ref);
        if let Some(config_input_reaction) = mode_action {
            use input_reaction_model::config::InputReaction::*;
            match config_input_reaction {
                SequenceNameString(sequence_name_string) => {
                    let sequence_id =
                        Self::sequence_id(sequence_id_mappings, &sequence_name_string);
                    input_reactions_loaded.push(fn_input_reaction(
                        sequence_id,
                        InputReactionAppEvents::default(),
                        IRR::default(),
                    ));
                }
                Single(InputReactionSingle {
                    next: sequence_name_string,
                    events,
                    requirement,
                }) => {
                    let sequence_id =
                        Self::sequence_id(sequence_id_mappings, &sequence_name_string);
                    input_reactions_loaded.push(fn_input_reaction(
                        sequence_id,
                        events.clone(),
                        requirement.clone(),
                    ));
                }
                Multiple(multiple) => input_reactions_loaded.extend(multiple.iter().map(
                    |InputReactionSingle {
                         next: sequence_name_string,
                         events,
                         requirement,
                     }| {
                        let sequence_id =
                            Self::sequence_id(sequence_id_mappings, &sequence_name_string);
                        fn_input_reaction(sequence_id, events.clone(), requirement.clone())
                    },
                )),
            }
        }
    }

    fn sequence_id(
        sequence_id_mappings: &SequenceIdMappings<SeqName>,
        sequence_name_string: &SequenceNameString<SeqName>,
    ) -> SequenceId {
        sequence_id_mappings
            .id(sequence_name_string)
            .copied()
            .unwrap_or_else(|| {
                panic!(
                    "Expected `sequence_id_mappings` to contain mapping for \
                     `{}`",
                    sequence_name_string
                )
            })
    }
}
