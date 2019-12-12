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
        config::{CharacterIrr, CharacterIrrPart, CharacterSequence, CharacterSequenceName},
        loaded::{CharacterInputReactions, CharacterIrs, CharacterIrsHandle},
    };
    use charge_model::config::ChargePoints;
    use game_input_model::{config::InputDirection, Axis, ControlAction};
    use input_reaction_model::{
        config::InputReactionAppEvents,
        loaded::{
            ActionHold, ActionPress, ActionRelease, AxisTransition, FallbackTransition,
            InputReaction, InputReactions, ReactionEffect,
        },
    };
    use object_model::play::{HealthPoints, SkillPoints};
    use pretty_assertions::assert_eq;
    use sequence_loading::SequenceLoadingBundle;
    use sequence_model::{
        config::SequenceNameString,
        loaded::{SequenceId, SequenceIdMappings},
    };

    use character_loading::{CharacterLoadingBundle, CHARACTER_INPUT_REACTIONS_DEFAULT};
    use input_reaction_loading::{InputReactionLoadingBundle, IrsLoader, IrsLoaderParams};

    #[test]
    fn loads_irses() -> Result<(), Error> {
        let sequence_default = CHARACTER_INPUT_REACTIONS_DEFAULT
            .object_definition
            .sequences
            .get(&SequenceNameString::Name(CharacterSequenceName::Stand));

        run_test(
            test_character_sequence(),
            sequence_default,
            |character_irs, input_reactions_assets| {
                let expected_character_input_reactions = expected_input_reactions_0();
                let character_input_reactions_handle = character_irs
                    .get(0)
                    .expect("Expected `CharacterInputReactionsHandle` to exist.");
                let character_input_reactions = input_reactions_assets
                    .get(character_input_reactions_handle)
                    .expect("Expected `CharacterInputReactions` to be loaded.");
                assert_eq!(
                    &expected_character_input_reactions,
                    character_input_reactions
                );

                let expected_character_input_reactions = expected_input_reactions_1();
                let character_input_reactions_handle = character_irs
                    .get(1)
                    .expect("Expected `CharacterInputReactionsHandle` to exist.");
                let character_input_reactions = input_reactions_assets
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
        assertion_fn: fn(&CharacterIrs, &AssetStorage<CharacterInputReactions>),
    ) -> Result<(), Error> {
        AmethystApplication::blank()
            .with_bundle(TransformBundle::new())
            .with_bundle(RenderEmptyBundle::<DefaultBackend>::new())
            .with_bundle(SequenceLoadingBundle::new())
            .with_bundle(CharacterLoadingBundle::new())
            .with_bundle(InputReactionLoadingBundle::new())
            .with_effect(move |world| {
                let character_irs_handle = {
                    let (loader, input_reactions_assets, input_reactions_sequence_assets) =
                        world.system_data::<TestSystemData>();
                    let character_loader_params = IrsLoaderParams {
                        loader: &loader,
                        input_reactions_assets: &input_reactions_assets,
                        input_reactions_sequence_assets: &input_reactions_sequence_assets,
                    };

                    IrsLoader::load(
                        &character_loader_params,
                        &sequence_id_mappings(),
                        sequence_default,
                        &sequence,
                    )
                };
                world.insert(character_irs_handle);
            })
            .with_effect(|_| {}) // Allow texture to load.
            .with_assertion(move |world| {
                let character_irs_handle = world.read_resource::<CharacterIrsHandle>().clone();
                let input_reactions_sequence_assets =
                    world.read_resource::<AssetStorage<CharacterIrs>>();
                let character_irs = input_reactions_sequence_assets
                    .get(&character_irs_handle)
                    .expect("Expected `CharacterIrs` to be loaded.");

                // Assert the values for each handle.
                let input_reactions_assets =
                    world.read_resource::<AssetStorage<CharacterInputReactions>>();

                assertion_fn(character_irs, &input_reactions_assets);
            })
            .run_isolated()
    }

    fn test_character_sequence() -> CharacterSequence {
        let test_character_sequence_yaml = "test_character_sequence.yaml";
        let test_character_sequence_path = PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "input_reaction_loading",
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

    // Should overwrite and inherit sequence input reactions.
    fn expected_input_reactions_0() -> CharacterInputReactions {
        InputReactions::new(vec![
            InputReaction {
                effect: ReactionEffect::ActionPress(ActionPress {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(5),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::ActionRelease(ActionRelease {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(1),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::new(vec![CharacterIrrPart::Charge(ChargePoints::new(
                    90,
                ))]),
            },
            InputReaction {
                effect: ReactionEffect::ActionRelease(ActionRelease {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(2),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::new(vec![CharacterIrrPart::Sp(SkillPoints::new(50))]),
            },
            InputReaction {
                effect: ReactionEffect::ActionRelease(ActionRelease {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(3),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::new(vec![CharacterIrrPart::Hp(HealthPoints::new(30))]),
            },
            InputReaction {
                effect: ReactionEffect::ActionHold(ActionHold {
                    action: ControlAction::Jump,
                    sequence_id: SequenceId::new(7),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::new(vec![
                    CharacterIrrPart::Charge(ChargePoints::new(90)),
                    CharacterIrrPart::Sp(SkillPoints::new(50)),
                ]),
            },
            InputReaction {
                effect: ReactionEffect::ActionHold(ActionHold {
                    action: ControlAction::Special,
                    sequence_id: SequenceId::new(8),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisPress(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(9),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisPress(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(12),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisRelease(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(11),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisRelease(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(14),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisHold(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(10),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisHold(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(13),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::Fallback(FallbackTransition {
                    sequence_id: SequenceId::new(3),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::new(vec![CharacterIrrPart::InputDirX(
                    InputDirection::NotSame,
                )]),
            },
        ])
    }

    // Should inherit from sequence input reactions.
    fn expected_input_reactions_1() -> CharacterInputReactions {
        InputReactions::new(vec![
            InputReaction {
                effect: ReactionEffect::ActionPress(ActionPress {
                    action: ControlAction::Attack,
                    sequence_id: SequenceId::new(4),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::ActionHold(ActionHold {
                    action: ControlAction::Jump,
                    sequence_id: SequenceId::new(6),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::ActionHold(ActionHold {
                    action: ControlAction::Special,
                    sequence_id: SequenceId::new(8),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisPress(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(9),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisPress(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(12),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisRelease(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(11),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisRelease(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(14),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisHold(AxisTransition {
                    axis: Axis::X,
                    sequence_id: SequenceId::new(10),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::AxisHold(AxisTransition {
                    axis: Axis::Z,
                    sequence_id: SequenceId::new(13),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::default(),
            },
            InputReaction {
                effect: ReactionEffect::Fallback(FallbackTransition {
                    sequence_id: SequenceId::new(3),
                    events: InputReactionAppEvents::default(),
                }),
                requirement: CharacterIrr::new(vec![CharacterIrrPart::InputDirX(
                    InputDirection::NotSame,
                )]),
            },
        ])
    }

    type TestSystemData<'s> = (
        ReadExpect<'s, Loader>,
        Read<'s, AssetStorage<CharacterInputReactions>>,
        Read<'s, AssetStorage<CharacterIrs>>,
    );
}
