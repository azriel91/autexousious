#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Read, ReadExpect, World, WorldExt, WriteStorage},
        shred::{ResourceId, SystemData},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application::IoUtils;
    use application_test_support::AutexousiousApplication;
    use character_loading::{IrsLoader, IrsLoaderParams};
    use character_model::{
        config::{CharacterIrr, CharacterSequence, CharacterSequenceName},
        loaded::{
            CharacterInputReactions, CharacterInputReactionsHandle, CharacterIrs,
            CharacterIrsHandle,
        },
    };
    use charge_model::{
        config::ChargePoints,
        play::{ChargeTrackerClock, ChargeUseEvent},
    };
    use derivative::Derivative;
    use game_input::ControllerInput;
    use game_input_model::{
        Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
    };
    use object_model::play::{HealthPoints, Mirrored, SkillPoints};
    use sequence_model::{
        config::SequenceNameString,
        loaded::{SequenceId, SequenceIdMappings},
    };

    use input_reaction_play::InputReactionsTransitionSystem;

    #[test]
    fn inserts_transition_for_action_press_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: Some(|entity| {
                    let control_action_event_data = ControlActionEventData {
                        entity,
                        control_action: ControlAction::Attack,
                    };
                    ControlInputEvent::ControlActionPress(control_action_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(4),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_action_release_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: Some(|entity| {
                    let control_action_event_data = ControlActionEventData {
                        entity,
                        control_action: ControlAction::Special,
                    };
                    ControlInputEvent::ControlActionRelease(control_action_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(9),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_action_hold() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.defend = true;
        controller_input.x_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(10),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn prioritizes_press_over_hold_transition() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: Some(|entity| {
                    let control_action_event_data = ControlActionEventData {
                        entity,
                        control_action: ControlAction::Jump,
                    };
                    ControlInputEvent::ControlActionPress(control_action_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(6),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn prioritizes_release_over_hold_transition() -> Result<(), Error> {
        // hold `Defend` but release `Special`
        let mut controller_input = ControllerInput::default();
        controller_input.defend = true;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: Some(|entity| {
                    let control_action_event_data = ControlActionEventData {
                        entity,
                        control_action: ControlAction::Special,
                    };
                    ControlInputEvent::ControlActionRelease(control_action_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(9),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_axis_press_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: Some(|entity| {
                    let axis_move_event_data = AxisMoveEventData {
                        entity,
                        axis: Axis::Z,
                        value: -1.,
                    };
                    ControlInputEvent::AxisMoved(axis_move_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(13),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_axis_release_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: Some(|entity| {
                    let axis_move_event_data = AxisMoveEventData {
                        entity,
                        axis: Axis::Z,
                        value: 0.,
                    };
                    ControlInputEvent::AxisMoved(axis_move_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(15),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_axis_hold() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(14),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn prioritizes_axis_press_over_hold_transition() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: Some(|entity| {
                    let axis_move_event_data = AxisMoveEventData {
                        entity,
                        axis: Axis::Z,
                        value: 1.,
                    };
                    ControlInputEvent::AxisMoved(axis_move_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(13),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn prioritizes_axis_release_over_hold_transition() -> Result<(), Error> {
        // hold `Z` but release `X`
        let mut controller_input = ControllerInput::default();
        controller_input.z_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: Some(|entity| {
                    let axis_move_event_data = AxisMoveEventData {
                        entity,
                        axis: Axis::X,
                        value: 0.,
                    };
                    ControlInputEvent::AxisMoved(axis_move_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(12),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_fallback() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(3),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn does_not_insert_transition_for_fallback_when_requirement_not_met() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.x_axis_value = 1.;

        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input,
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(0),
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn sends_charge_use_event_when_requirement_met() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: Some(|entity| {
                    let control_action_event_data = ControlActionEventData {
                        entity,
                        control_action: ControlAction::Special,
                    };
                    ControlInputEvent::ControlActionRelease(control_action_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(9),
                charge_use_events_fn: Some(|entity| {
                    let charge_use_event = ChargeUseEvent {
                        entity,
                        charge_points: ChargePoints::new(10),
                    };
                    vec![charge_use_event]
                }),
            },
        )
    }

    #[test]
    fn does_not_send_charge_use_event_when_requirement_not_met() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: SequenceId::new(0),
                controller_input: ControllerInput::default(),
                control_input_event_fn: Some(|entity| {
                    let control_action_event_data = ControlActionEventData {
                        entity,
                        control_action: ControlAction::Special,
                    };
                    ControlInputEvent::ControlActionRelease(control_action_event_data)
                }),
                charge_tracker_clock: ChargeTrackerClock::new_with_value(5, 5),
            },
            ExpectedParams {
                sequence_id: SequenceId::new(0),
                charge_use_events_fn: Some(|_| vec![]),
            },
        )
    }

    fn run_test(
        SetupParams {
            sequence_id: sequence_id_setup,
            controller_input: controller_input_setup,
            control_input_event_fn,
            charge_tracker_clock: charge_tracker_clock_setup,
        }: SetupParams,
        ExpectedParams {
            sequence_id: sequence_id_expected,
            charge_use_events_fn,
        }: ExpectedParams,
    ) -> Result<(), Error> {
        AutexousiousApplication::config_base()
            .with_system(
                InputReactionsTransitionSystem::<CharacterIrr>::new(),
                "",
                &[],
            )
            .with_effect(register_reader)
            .with_effect(move |world| {
                let character_irs_handle = {
                    let (loader, character_input_reactions_assets, character_irs_assets) = world
                        .system_data::<(
                            ReadExpect<'_, Loader>,
                            Read<'_, AssetStorage<CharacterInputReactions>>,
                            Read<'_, AssetStorage<CharacterIrs>>,
                        )>();

                    let irs_loader_params = IrsLoaderParams {
                        loader: &loader,
                        character_input_reactions_assets: &character_input_reactions_assets,
                        character_irs_assets: &character_irs_assets,
                    };
                    let test_character_sequence = test_character_sequence();

                    IrsLoader::load(
                        &irs_loader_params,
                        &sequence_id_mappings(),
                        None,
                        &test_character_sequence,
                    )
                };

                world.insert(character_irs_handle);
            })
            // Allow `AssetStorage`s to process loaded data.
            .with_effect(move |world| {
                let character_input_reactions_handle = {
                    let character_irs_assets =
                        world.system_data::<Read<'_, AssetStorage<CharacterIrs>>>();

                    let character_irs_handle = world.read_resource::<CharacterIrsHandle>().clone();
                    let character_irs = character_irs_assets
                        .get(&character_irs_handle)
                        .expect("Expected `character_irs` to be loaded.");
                    character_irs
                        .first()
                        .expect(
                            "Expected `character_irs` to contain one \
                             `character_input_reactions_handle`.",
                        )
                        .clone()
                };

                let entity = world.create_entity().build();
                {
                    let TestSystemData {
                        mut sequence_ids,
                        mut character_input_reactions_handles,
                        mut health_pointses,
                        mut skill_pointses,
                        mut charge_tracker_clocks,
                        mut mirroreds,
                        mut controller_inputs,
                    } = world.system_data::<TestSystemData>();

                    sequence_ids
                        .insert(entity, sequence_id_setup)
                        .expect("Failed to insert `SequenceId` component.");
                    character_input_reactions_handles
                        .insert(entity, character_input_reactions_handle)
                        .expect("Failed to insert `CharacterInputReactionsHandle` component.");
                    health_pointses
                        .insert(entity, HealthPoints::new(100))
                        .expect("Failed to insert `HealthPoints` component.");
                    skill_pointses
                        .insert(entity, SkillPoints::new(100))
                        .expect("Failed to insert `SkillPoints` component.");
                    charge_tracker_clocks
                        .insert(entity, charge_tracker_clock_setup)
                        .expect("Failed to insert `ChargeTrackerClock` component.");
                    mirroreds
                        .insert(entity, Mirrored::new(false))
                        .expect("Failed to insert `Mirrored` component.");

                    controller_inputs
                        .insert(entity, controller_input_setup)
                        .expect("Failed to insert `ControllerInput` component.");
                }

                if let Some(control_input_event_fn) = control_input_event_fn {
                    send_event(world, control_input_event_fn(entity));
                }

                world.insert(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();

                let sequence_id = {
                    let sequence_ids = world.read_storage::<SequenceId>();

                    sequence_ids
                        .get(entity)
                        .copied()
                        .expect("Expected `SequenceId` component to exist.")
                };

                assert_eq!(sequence_id_expected, sequence_id);

                if let Some(charge_use_events_fn) = charge_use_events_fn {
                    let charge_use_events = charge_use_events_fn(entity);

                    expect_events(world, charge_use_events);
                }
            })
            .run_isolated()
    }

    fn test_character_sequence() -> CharacterSequence {
        let test_character_sequence_yaml = "test_character_sequence.yaml";
        let test_character_sequence_path = PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "src",
            "input_reaction_play",
            "system",
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
            SequenceNameString::Name(CharacterSequenceName::DashBack),
            SequenceId::new(9),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Flinch0),
            SequenceId::new(10),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Flinch1),
            SequenceId::new(11),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::Dazed),
            SequenceId::new(12),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::FallForwardAscend),
            SequenceId::new(13),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::FallForwardDescend),
            SequenceId::new(14),
        );
        sequence_id_mappings.insert(
            SequenceNameString::Name(CharacterSequenceName::LieFaceDown),
            SequenceId::new(15),
        );
        sequence_id_mappings
    }

    fn register_reader(world: &mut World) {
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<ChargeUseEvent>>();
            ec.register_reader()
        }; // kcov-ignore
        world.insert(reader_id);
    }

    fn send_event(world: &mut World, event: ControlInputEvent) {
        let mut ec = world.write_resource::<EventChannel<ControlInputEvent>>();
        ec.single_write(event);
    } // kcov-ignore

    fn expect_events(world: &mut World, events_expected: Vec<ChargeUseEvent>) {
        let target_entity = *world.read_resource::<Entity>();
        let mut reader_id = world.write_resource::<ReaderId<ChargeUseEvent>>();
        let ec = world.read_resource::<EventChannel<ChargeUseEvent>>();

        // Map owned values into references.
        let events_expected = events_expected.iter().collect::<Vec<_>>();

        // Filter events for the entity we care about.
        let events_actual = ec
            .read(&mut reader_id)
            .filter(|ev| target_entity == ev.entity)
            .collect::<Vec<_>>();

        assert_eq!(events_expected, events_actual)
    }

    #[derive(Derivative, SystemData)]
    #[derivative(Debug)]
    struct TestSystemData<'s> {
        #[derivative(Debug = "ignore")]
        sequence_ids: WriteStorage<'s, SequenceId>,
        #[derivative(Debug = "ignore")]
        character_input_reactions_handles: WriteStorage<'s, CharacterInputReactionsHandle>,
        #[derivative(Debug = "ignore")]
        health_pointses: WriteStorage<'s, HealthPoints>,
        #[derivative(Debug = "ignore")]
        skill_pointses: WriteStorage<'s, SkillPoints>,
        #[derivative(Debug = "ignore")]
        charge_tracker_clocks: WriteStorage<'s, ChargeTrackerClock>,
        #[derivative(Debug = "ignore")]
        mirroreds: WriteStorage<'s, Mirrored>,
        #[derivative(Debug = "ignore")]
        controller_inputs: WriteStorage<'s, ControllerInput>,
    }

    struct SetupParams {
        sequence_id: SequenceId,
        controller_input: ControllerInput,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
        charge_tracker_clock: ChargeTrackerClock,
    }

    struct ExpectedParams {
        sequence_id: SequenceId,
        charge_use_events_fn: Option<fn(Entity) -> Vec<ChargeUseEvent>>,
    }
}
