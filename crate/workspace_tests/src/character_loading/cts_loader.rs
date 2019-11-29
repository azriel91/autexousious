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
    use application::IoUtils;
    use character_model::{
        config::{CharacterSequence, CharacterSequenceName, ControlTransitionRequirement},
        loaded::{
            CharacterControlTransition, CharacterCts, CharacterCtsHandle, CharacterInputReactions,
        },
    };
    use charge_model::config::ChargePoints;
    use game_input_model::{config::InputDirection, Axis, ControlAction};
    use object_model::play::{HealthPoints, SkillPoints};
    use pretty_assertions::assert_eq;
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::{
        config::SequenceNameString,
        loaded::{
            ActionHold, ActionPress, ActionRelease, AxisTransition, ControlTransition,
            FallbackTransition, InputReactions, SequenceId, SequenceIdMappings,
        },
    };

    use character_loading::{
        CharacterLoadingBundle, CtsLoader, CtsLoaderParams, CHARACTER_TRANSITIONS_DEFAULT,
    };

    #[test]
    fn loads_ctss() -> Result<(), Error> {
        let sequence_default = CHARACTER_TRANSITIONS_DEFAULT
            .object_definition
            .sequences
            .get(&SequenceNameString::Name(CharacterSequenceName::Stand));

        run_test(
            test_character_sequence(),
            sequence_default,
            |character_cts, character_input_reactions_assets| {
                let expected_character_input_reactions = expected_input_reactions_0();
                let character_input_reactions_handle = character_cts
                    .get(0)
                    .expect("Expected `CharacterInputReactionsHandle` to exist.");
                let character_input_reactions = character_input_reactions_assets
                    .get(character_input_reactions_handle)
                    .expect("Expected `CharacterInputReactions` to be loaded.");
                assert_eq!(
                    &expected_character_input_reactions,
                    character_input_reactions
                );

                let expected_character_input_reactions = expected_input_reactions_1();
                let character_input_reactions_handle = character_cts
                    .get(1)
                    .expect("Expected `CharacterInputReactionsHandle` to exist.");
                let character_input_reactions = character_input_reactions_assets
                    .get(character_input_reactions_handle)
                    .expect("Expected `CharacterInputReactions` to be loaded.");
                assert_eq!(
                    &expected_character_input_reactions,
                    character_input_reactions
                );
            },
        )
    }

    fn run_test(
        sequence: CharacterSequence,
        sequence_default: Option<&'static CharacterSequence>,
        assertion_fn: fn(&CharacterCts, &AssetStorage<CharacterInputReactions>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_effect(move |world| {
                let character_cts_handle = {
                    let (loader, character_input_reactions_assets, character_cts_assets) =
                        world.system_data::<TestSystemData>();
                    let character_loader_params = CtsLoaderParams {
                        loader: &loader,
                        character_input_reactions_assets: &character_input_reactions_assets,
                        character_cts_assets: &character_cts_assets,
                    };

                    CtsLoader::load(
                        &character_loader_params,
                        &sequence_id_mappings(),
                        sequence_default,
                        &sequence,
                    )
                };
                world.insert(character_cts_handle);
            })
            .with_effect(|_| {}) // Allow texture to load.
            .with_assertion(move |world| {
                let character_cts_handle = world.read_resource::<CharacterCtsHandle>().clone();
                let character_cts_assets = world.read_resource::<AssetStorage<CharacterCts>>();
                let character_cts = character_cts_assets
                    .get(&character_cts_handle)
                    .expect("Expected `CharacterCts` to be loaded.");

                // Assert the values for each handle.
                let character_input_reactions_assets =
                    world.read_resource::<AssetStorage<CharacterInputReactions>>();

                assertion_fn(character_cts, &character_input_reactions_assets);
            })
            .run_isolated()
    }

    fn test_character_sequence() -> CharacterSequence {
        let test_character_sequence_yaml = "test_character_sequence.yaml";
        let test_character_sequence_path = PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "character_loading",
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
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Stand),
            SequenceId::new(0),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Walk),
            SequenceId::new(1),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Run),
            SequenceId::new(2),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::RunStop),
            SequenceId::new(3),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::StandAttack0),
            SequenceId::new(4),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::StandAttack1),
            SequenceId::new(5),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Jump),
            SequenceId::new(6),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::JumpOff),
            SequenceId::new(7),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::DashForward),
            SequenceId::new(8),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Flinch0),
            SequenceId::new(9),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Flinch1),
            SequenceId::new(10),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Dazed),
            SequenceId::new(11),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::FallForwardAscend),
            SequenceId::new(12),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::FallForwardDescend),
            SequenceId::new(13),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::LieFaceDown),
            SequenceId::new(14),
        );
        sequence_id_mappings
    }

    // Should overwrite and inherit sequence transitions.
    fn expected_input_reactions_0() -> CharacterInputReactions {
        CharacterInputReactions::new(InputReactions::new(vec![
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
    fn expected_input_reactions_1() -> CharacterInputReactions {
        CharacterInputReactions::new(InputReactions::new(vec![
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
        Read<'s, AssetStorage<CharacterInputReactions>>,
        Read<'s, AssetStorage<CharacterCts>>,
    );
}
