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
                    &sequence.transitions,
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
        config_transitions_sequence: &config::CharacterControlTransitions,
        config_transitions_frame: &config::CharacterControlTransitions,
    ) -> loaded::CharacterControlTransitionsHandle {
        let mut loaded_transitions = Vec::new();

        macro_rules! push_transitions {
            ($mode_action:ident, $mode:ident, $mode_data:ident, $action:ident) => {
                let mode_action = config_transitions_frame
                    .$mode_action
                    .as_ref()
                    .or(config_transitions_sequence.$mode_action.as_ref());
                if let Some(config_control_transition) = &mode_action {
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

#[cfg(test)]
mod tests {
    use amethyst::{
        assets::{AssetStorage, Loader},
        core::TransformBundle,
        ecs::{Read, ReadExpect},
        renderer::{types::DefaultBackend, RenderEmptyBundle},
        Error,
    };
    use amethyst_test::AmethystApplication;
    use character_model::{
        config::{CharacterSequence, CharacterSequenceId, ControlTransitionRequirement},
        loaded::{
            CharacterControlTransition, CharacterControlTransitions,
            CharacterControlTransitionsSequence, CharacterControlTransitionsSequenceHandle,
        },
    };
    use game_input_model::ControlAction;
    use object_model::{
        config::{ObjectFrame, ObjectSequence},
        play::{ChargePoints, HealthPoints, SkillPoints},
    };
    use pretty_assertions::assert_eq;
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::{
        config::SequenceEndTransition,
        loaded::{
            ControlTransition, ControlTransitionHold, ControlTransitionPress,
            ControlTransitionRelease, ControlTransitions,
        },
    };

    use super::CharacterLoader;
    use crate::{CharacterLoaderParams, CharacterLoadingBundle};

    #[test]
    fn loads_control_transitions_sequences() -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_setup(|world| {
                let character_cts_handle = {
                    let (
                        loader,
                        character_control_transitions_assets,
                        character_control_transitions_sequence_assets,
                    ) = world.system_data::<TestSystemData>();
                    let character_loader_params = CharacterLoaderParams {
                        loader: &loader,
                        character_control_transitions_assets: &character_control_transitions_assets,
                        character_control_transitions_sequence_assets:
                            &character_control_transitions_sequence_assets,
                    };
                    let character_sequence = character_sequence();

                    CharacterLoader::control_transitions_sequence_handle(
                        &character_loader_params,
                        &character_sequence,
                    )
                };
                world.add_resource(character_cts_handle);
            })
            .with_setup(|_world| {}) // Allow texture to load.
            .with_assertion(|world| {
                let character_cts_handle = world
                    .read_resource::<CharacterControlTransitionsSequenceHandle>()
                    .clone();
                let character_control_transitions_sequence_assets =
                    world.read_resource::<AssetStorage<CharacterControlTransitionsSequence>>();
                let character_control_transitions_sequences =
                    character_control_transitions_sequence_assets
                        .get(&character_cts_handle)
                        .expect("Expected `CharacterControlTransitionsSequence` to be loaded.");

                // Assert the values for each handle.
                let character_control_transitions_assets =
                    world.read_resource::<AssetStorage<CharacterControlTransitions>>();

                let expected_character_control_transitions = expected_control_transitions_0();
                let character_control_transitions_handle = character_control_transitions_sequences
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
                let character_control_transitions_handle = character_control_transitions_sequences
                    .get(1)
                    .expect("Expected `CharacterControlTransitionsHandle` to exist.");
                let character_control_transitions = character_control_transitions_assets
                    .get(character_control_transitions_handle)
                    .expect("Expected `CharacterControlTransitions` to be loaded.");
                assert_eq!(
                    &expected_character_control_transitions,
                    character_control_transitions
                );
            })
            .run_isolated()
    }

    fn character_sequence() -> CharacterSequence {
        use character_model::config::{CharacterControlTransitions, CharacterFrame};
        use sequence_model::config::{
            ControlTransition, ControlTransitionMultiple, ControlTransitionSingle, Wait,
        };

        let frames = vec![
            CharacterFrame::new(
                ObjectFrame {
                    wait: Wait::new(5),
                    ..Default::default()
                },
                CharacterControlTransitions {
                    press_attack: Some(ControlTransition::SequenceId(
                        CharacterSequenceId::StandAttack0,
                    )),
                    release_attack: Some(ControlTransition::Multiple(
                        ControlTransitionMultiple::new(vec![
                            ControlTransitionSingle {
                                next: CharacterSequenceId::Walk,
                                requirements: vec![ControlTransitionRequirement::Charge(
                                    ChargePoints::new(90),
                                )],
                            },
                            ControlTransitionSingle {
                                next: CharacterSequenceId::Run,
                                requirements: vec![ControlTransitionRequirement::Sp(
                                    SkillPoints::new(50),
                                )],
                            },
                            ControlTransitionSingle {
                                next: CharacterSequenceId::RunStop,
                                requirements: vec![ControlTransitionRequirement::Hp(
                                    HealthPoints::new(30),
                                )],
                            },
                        ]),
                    )),
                    hold_jump: Some(ControlTransition::Single(ControlTransitionSingle {
                        next: CharacterSequenceId::Jump,
                        requirements: vec![],
                    })),
                    ..Default::default()
                }, // kcov-ignore
            ),
            CharacterFrame::new(
                ObjectFrame::default(),
                CharacterControlTransitions::default(),
            ),
        ];

        CharacterSequence::new(
            ObjectSequence::new(
                SequenceEndTransition::SequenceId(CharacterSequenceId::Stand),
                frames,
            ),
            CharacterControlTransitions {
                press_attack: Some(ControlTransition::SequenceId(
                    CharacterSequenceId::StandAttack1,
                )),
                hold_special: Some(ControlTransition::SequenceId(
                    CharacterSequenceId::DashForward,
                )),
                ..Default::default()
            }, // kcov-ignore
        )
    }

    // Should overwrite and inherit sequence transitions.
    fn expected_control_transitions_0() -> CharacterControlTransitions {
        CharacterControlTransitions::new(ControlTransitions::new(vec![
            CharacterControlTransition {
                control_transition: ControlTransition::Press(ControlTransitionPress {
                    action: ControlAction::Attack,
                    sequence_id: CharacterSequenceId::StandAttack0,
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Release(ControlTransitionRelease {
                    action: ControlAction::Attack,
                    sequence_id: CharacterSequenceId::Walk,
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::Charge(
                    ChargePoints::new(90),
                )],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Release(ControlTransitionRelease {
                    action: ControlAction::Attack,
                    sequence_id: CharacterSequenceId::Run,
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::Sp(
                    SkillPoints::new(50),
                )],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Release(ControlTransitionRelease {
                    action: ControlAction::Attack,
                    sequence_id: CharacterSequenceId::RunStop,
                }),
                control_transition_requirements: vec![ControlTransitionRequirement::Hp(
                    HealthPoints::new(30),
                )],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Hold(ControlTransitionHold {
                    action: ControlAction::Jump,
                    sequence_id: CharacterSequenceId::Jump,
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Hold(ControlTransitionHold {
                    action: ControlAction::Special,
                    sequence_id: CharacterSequenceId::DashForward,
                }),
                control_transition_requirements: vec![],
            },
        ]))
    }

    // Should inherit from sequence transitions.
    fn expected_control_transitions_1() -> CharacterControlTransitions {
        CharacterControlTransitions::new(ControlTransitions::new(vec![
            CharacterControlTransition {
                control_transition: ControlTransition::Press(ControlTransitionPress {
                    action: ControlAction::Attack,
                    sequence_id: CharacterSequenceId::StandAttack1,
                }),
                control_transition_requirements: vec![],
            },
            CharacterControlTransition {
                control_transition: ControlTransition::Hold(ControlTransitionHold {
                    action: ControlAction::Special,
                    sequence_id: CharacterSequenceId::DashForward,
                }),
                control_transition_requirements: vec![],
            },
        ]))
    }

    type TestSystemData<'s> = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<CharacterControlTransitions>>,
        Read<'s, AssetStorage<CharacterControlTransitionsSequence>>,
    );
}
