use amethyst::assets::{AssetStorage, Loader};
use character_model::{
    config::{self, CharacterSequence, CharacterSequenceName},
    loaded::{
        self, CharacterControlTransition, CharacterControlTransitionsSequence,
        CharacterControlTransitionsSequenceHandle,
    },
};
use game_input_model::{Axis, ControlAction};
use object_model::config::GameObjectSequence;
use sequence_model::{
    config::ControlTransitionSingle,
    loaded::{
        ActionHold, ActionPress, ActionRelease, AxisTransition, ControlTransition,
        ControlTransitions, FallbackTransition, SequenceIdMappings,
    },
};

use crate::ControlTransitionsSequenceLoaderParams;

/// Loads control transitions configuration into the loaded model.
#[derive(Debug)]
pub enum ControlTransitionsSequenceLoader {}

impl ControlTransitionsSequenceLoader {
    /// Extracts a `CharacterControlTransitionsSequence` from a `CharacterSequence`.
    pub fn load(
        ControlTransitionsSequenceLoaderParams {
            loader,
            character_control_transitions_assets,
            character_control_transitions_sequence_assets,
        }: &ControlTransitionsSequenceLoaderParams,
        sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>,
        sequence_default: Option<&CharacterSequence>,
        sequence: &CharacterSequence,
    ) -> CharacterControlTransitionsSequenceHandle {
        let control_transitions_sequence = sequence
            .object_sequence()
            .frames
            .iter()
            .map(|frame| {
                let config_transitions_default =
                    sequence_default.and_then(|sequence| sequence.transitions.as_ref());
                Self::config_to_loaded_transitions_handle(
                    loader,
                    character_control_transitions_assets,
                    sequence_id_mappings,
                    config_transitions_default,
                    sequence.transitions.as_ref(),
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
        sequence_id_mappings: &SequenceIdMappings<CharacterSequenceName>,
        config_transitions_default: Option<&config::CharacterControlTransitions>,
        config_transitions_sequence: Option<&config::CharacterControlTransitions>,
        config_transitions_frame: &config::CharacterControlTransitions,
    ) -> loaded::CharacterControlTransitionsHandle {
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
                    use sequence_model::config::ControlTransition::*;
                    match config_control_transition {
                        SequenceName(sequence_name) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name
                                    )
                                });
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
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_id).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_id
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                ControlTransition::$mode($mode_data {
                                    action: ControlAction::$action,
                                    sequence_id: *sequence_id,
                                }),
                                control_transition_requirements.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_id,
                                 requirements: control_transition_requirements,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_id).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_id
                                    )
                                });
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
                    use sequence_model::config::ControlTransition::*;
                    match config_control_transition {
                        SequenceName(sequence_name) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                ControlTransition::$mode(AxisTransition {
                                    axis: Axis::$axis,
                                    sequence_id: *sequence_id,
                                }),
                                vec![],
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_id,
                            requirements: control_transition_requirements,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_id).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_id
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                ControlTransition::$mode(AxisTransition {
                                    axis: Axis::$axis,
                                    sequence_id: *sequence_id,
                                }),
                                control_transition_requirements.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_id,
                                 requirements: control_transition_requirements,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_id).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_id
                                    )
                                });
                                CharacterControlTransition::new(
                                    ControlTransition::$mode(AxisTransition {
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
                    use sequence_model::config::ControlTransition::*;
                    match config_control_transition {
                        SequenceName(sequence_name) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_name).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_name
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                ControlTransition::$mode(FallbackTransition {
                                    sequence_id: *sequence_id,
                                }),
                                vec![],
                            ));
                        }
                        Single(ControlTransitionSingle {
                            next: sequence_id,
                            requirements: control_transition_requirements,
                        }) => {
                            let sequence_id =
                                sequence_id_mappings.id(sequence_id).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_id
                                    )
                                });
                            loaded_transitions.push(CharacterControlTransition::new(
                                ControlTransition::$mode(FallbackTransition {
                                    sequence_id: *sequence_id,
                                }),
                                control_transition_requirements.clone(),
                            ))
                        }
                        Multiple(multiple) => loaded_transitions.extend(multiple.iter().map(
                            |ControlTransitionSingle {
                                 next: sequence_id,
                                 requirements: control_transition_requirements,
                             }| {
                                let sequence_id =
                                sequence_id_mappings.id(sequence_id).unwrap_or_else(|| {
                                    panic!(
                                        "Expected `sequence_id_mappings` to contain mapping for \
                                         `{}`",
                                        sequence_id
                                    )
                                });
                                CharacterControlTransition::new(
                                    ControlTransition::$mode(FallbackTransition {
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

        let character_control_transitions =
            loaded::CharacterControlTransitions::new(ControlTransitions::new(loaded_transitions));

        loader.load_from_data(
            character_control_transitions,
            (),
            character_control_transitions_assets,
        )
    }
}

#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use amethyst::{
        assets::{AssetStorage, Loader},
        core::TransformBundle,
        ecs::{Read, ReadExpect, WorldExt},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use application::resource::IoUtils;
    use character_model::{
        config::{CharacterSequence, CharacterSequenceName, ControlTransitionRequirement},
        loaded::{
            CharacterControlTransition, CharacterControlTransitions,
            CharacterControlTransitionsSequence, CharacterControlTransitionsSequenceHandle,
        },
    };
    use charge_model::config::ChargePoints;
    use game_input_model::{config::InputDirection, Axis, ControlAction};
    use object_model::play::{HealthPoints, SkillPoints};
    use pretty_assertions::assert_eq;
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::loaded::{
        ActionHold, ActionPress, ActionRelease, AxisTransition, ControlTransition,
        ControlTransitions, FallbackTransition, SequenceId, SequenceIdMappings,
    };

    use super::ControlTransitionsSequenceLoader;
    use crate::{
        CharacterLoadingBundle, ControlTransitionsSequenceLoaderParams,
        CHARACTER_TRANSITIONS_DEFAULT,
    };

    #[test]
    fn loads_control_transitions_sequences() -> Result<(), Error> {
        let sequence_default = CHARACTER_TRANSITIONS_DEFAULT
            .object_definition
            .sequences
            .get(&CharacterSequenceName::Stand);

        run_test(
            test_character_sequence(),
            sequence_default,
            |character_control_transitions_sequence, character_control_transitions_assets| {
                let expected_character_control_transitions = expected_control_transitions_0();
                let character_control_transitions_handle = character_control_transitions_sequence
                    .get(0)
                    .expect("Expected `CharacterControlTransitionsHandle` to exist.");
                let character_control_transitions = character_control_transitions_assets
                    .get(character_control_transitions_handle)
                    .expect("Expected `CharacterControlTransitions` to be loaded.");
                assert_eq!(
                    &expected_character_control_transitions,
                    character_control_transitions
                );

                let expected_character_control_transitions = expected_control_transitions_1();
                let character_control_transitions_handle = character_control_transitions_sequence
                    .get(1)
                    .expect("Expected `CharacterControlTransitionsHandle` to exist.");
                let character_control_transitions = character_control_transitions_assets
                    .get(character_control_transitions_handle)
                    .expect("Expected `CharacterControlTransitions` to be loaded.");
                assert_eq!(
                    &expected_character_control_transitions,
                    character_control_transitions
                );
            },
        )
    }

    fn run_test(
        sequence: CharacterSequence,
        sequence_default: Option<&'static CharacterSequence>,
        assertion_fn: fn(
            &CharacterControlTransitionsSequence,
            &AssetStorage<CharacterControlTransitions>,
        ),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_setup(move |world| {
                let character_cts_handle = {
                    let (
                        loader,
                        character_control_transitions_assets,
                        character_control_transitions_sequence_assets,
                    ) = world.system_data::<TestSystemData>();
                    let character_loader_params = ControlTransitionsSequenceLoaderParams {
                        loader: &loader,
                        character_control_transitions_assets: &character_control_transitions_assets,
                        character_control_transitions_sequence_assets:
                            &character_control_transitions_sequence_assets,
                    };

                    ControlTransitionsSequenceLoader::load(
                        &character_loader_params,
                        &sequence_id_mappings(),
                        sequence_default,
                        &sequence,
                    )
                };
                world.insert(character_cts_handle);
            })
            .with_setup(|_world| {}) // Allow texture to load.
            .with_assertion(move |world| {
                let character_cts_handle = world
                    .read_resource::<CharacterControlTransitionsSequenceHandle>()
                    .clone();
                let character_control_transitions_sequence_assets =
                    world.read_resource::<AssetStorage<CharacterControlTransitionsSequence>>();
                let character_control_transitions_sequence =
                    character_control_transitions_sequence_assets
                        .get(&character_cts_handle)
                        .expect("Expected `CharacterControlTransitionsSequence` to be loaded.");

                // Assert the values for each handle.
                let character_control_transitions_assets =
                    world.read_resource::<AssetStorage<CharacterControlTransitions>>();

                assertion_fn(
                    character_control_transitions_sequence,
                    &character_control_transitions_assets,
                );
            })
            .run_isolated()
    }

    fn test_character_sequence() -> CharacterSequence {
        let test_character_sequence_yaml = "test_character_sequence.yaml";
        let test_character_sequence_path = PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            test_character_sequence_yaml,
        ]);
        let contents = IoUtils::read_file(&test_character_sequence_path).unwrap_or_else(|e| {
            panic!(
                "Failed to read `{}`. Error: {}",
                test_character_sequence_yaml, e
            )
        });

        serde_yaml::from_slice::<CharacterSequence>(&contents)
            .expect("Failed to load `test_character_sequence.yaml`.")
    }

    fn sequence_id_mappings() -> SequenceIdMappings<CharacterSequenceName> {
        let mut sequence_id_mappings = SequenceIdMappings::new();
        sequence_id_mappings.insert(CharacterSequenceName::Stand, SequenceId::new(0));
        sequence_id_mappings.insert(CharacterSequenceName::Walk, SequenceId::new(1));
        sequence_id_mappings.insert(CharacterSequenceName::Run, SequenceId::new(2));
        sequence_id_mappings.insert(CharacterSequenceName::RunStop, SequenceId::new(3));
        sequence_id_mappings.insert(CharacterSequenceName::StandAttack0, SequenceId::new(4));
        sequence_id_mappings.insert(CharacterSequenceName::StandAttack1, SequenceId::new(5));
        sequence_id_mappings.insert(CharacterSequenceName::Jump, SequenceId::new(6));
        sequence_id_mappings.insert(CharacterSequenceName::JumpOff, SequenceId::new(7));
        sequence_id_mappings.insert(CharacterSequenceName::DashForward, SequenceId::new(8));
        sequence_id_mappings.insert(CharacterSequenceName::Flinch0, SequenceId::new(9));
        sequence_id_mappings.insert(CharacterSequenceName::Flinch1, SequenceId::new(10));
        sequence_id_mappings.insert(CharacterSequenceName::Dazed, SequenceId::new(11));
        sequence_id_mappings.insert(
            CharacterSequenceName::FallForwardAscend,
            SequenceId::new(12),
        );
        sequence_id_mappings.insert(
            CharacterSequenceName::FallForwardDescend,
            SequenceId::new(13),
        );
        sequence_id_mappings.insert(CharacterSequenceName::LieFaceDown, SequenceId::new(14));
        sequence_id_mappings
    }

    // Should overwrite and inherit sequence transitions.
    fn expected_control_transitions_0() -> CharacterControlTransitions {
        CharacterControlTransitions::new(ControlTransitions::new(vec![
            CharacterControlTransition {
                control_transition: ControlTransition::ActionPress(ActionPress {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(5),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionRelease(ActionRelease {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(1),
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::Charge(
                    ChargePoints::new(90),
                )],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionRelease(ActionRelease {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(2),
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::Sp(
                    SkillPoints::new(50),
                )],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionRelease(ActionRelease {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(3),
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::Hp(
                    HealthPoints::new(30),
                )],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionHold(ActionHold {
                    action: ControlAction::Jump,
                    sequence_id: SequenceId::new(7),
                }),
                control_transition_requirements: vec![
                    ControlTransitionRequirement::Charge(ChargePoints::new(90)),
                    ControlTransitionRequirement::Sp(SkillPoints::new(50)),
                ],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionHold(ActionHold {
                    action: ControlAction::Special,
                    sequence_id: SequenceId::new(8),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisPress(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(9),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisPress(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(12),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisRelease(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(11),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisRelease(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(14),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisHold(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(10),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisHold(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(13),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Fallback(FallbackTransition {
                    sequence_id: SequenceId::new(3),
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::InputDirX(
                    InputDirection::NotSame,
                )],
            },
        ]))
    }

    // Should inherit from sequence transitions.
    fn expected_control_transitions_1() -> CharacterControlTransitions {
        CharacterControlTransitions::new(ControlTransitions::new(vec![
            CharacterControlTransition {
                control_transition: ControlTransition::ActionPress(ActionPress {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(4),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionHold(ActionHold {
                    action: ControlAction::Jump,
                    sequence_id: SequenceId::new(6),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::ActionHold(ActionHold {
                    action: ControlAction::Special,
                    sequence_id: SequenceId::new(8),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisPress(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(9),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisPress(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(12),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisRelease(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(11),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisRelease(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(14),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisHold(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(10),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::AxisHold(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(13),
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Fallback(FallbackTransition {
                    sequence_id: SequenceId::new(3),
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::InputDirX(
                    InputDirection::NotSame,
                )],
            },
        ]))
    }

    type TestSystemData<'s> = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<CharacterControlTransitions>>,
        Read<'s, AssetStorage<CharacterControlTransitionsSequence>>,
    );
}
