use amethyst::{
    assets::AssetStorage,
    ecs::{
        BitSet, Entities, Entity, Join, Read, ReadStorage, Resources, System, SystemData, Write,
        WriteStorage,
    },
    shrev::{EventChannel, ReaderId},
};
use approx::{relative_eq, relative_ne};
use character_model::{
    config::{CharacterSequenceId, ControlTransitionRequirement},
    loaded::{CharacterControlTransitions, CharacterControlTransitionsHandle},
};
use charge_model::play::ChargeUseEvent;
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use game_input_model::{
    Axis, AxisMoveEventData, ControlAction, ControlActionEventData, ControlInputEvent,
};
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::loaded::{
    ActionHold, ActionPress, ActionRelease, AxisTransition, ControlTransition,
    ControlTransitionLike, FallbackTransition,
};
use shred_derive::SystemData;

use crate::ControlTransitionRequirementSystemData;

/// Updates `CharacterSequenceId` based on `ControlInputEvent`s and held buttons.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterControlTransitionsTransitionSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
    /// Pre-allocated bitset to track entities whose transitions have already been checked.
    #[new(default)]
    processed_entities: BitSet,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterControlTransitionsTransitionSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `CharacterControlTransitionsTransitionResources`.
    pub character_control_transitions_transition_resources:
        CharacterControlTransitionsTransitionResources<'s>,
    /// `ControlTransitionRequirementSystemData`.
    pub control_transition_requirement_system_data: ControlTransitionRequirementSystemData<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterControlTransitionsTransitionResources<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `CharacterControlTransitionsHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_handles: ReadStorage<'s, CharacterControlTransitionsHandle>,
    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: Read<'s, AssetStorage<CharacterControlTransitions>>,
    /// `CharacterSequenceId` components.
    #[derivative(Debug = "ignore")]
    pub character_sequence_ids: WriteStorage<'s, CharacterSequenceId>,
    /// `ChargeUseEvent` channel.
    #[derivative(Debug = "ignore")]
    pub charge_use_ec: Write<'s, EventChannel<ChargeUseEvent>>,
}

impl CharacterControlTransitionsTransitionSystem {
    fn handle_action_event(
        &mut self,
        CharacterControlTransitionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref character_control_transitions_handles,
            ref character_control_transitions_assets,
            ref mut character_sequence_ids,
            ref mut charge_use_ec,
        }: &mut CharacterControlTransitionsTransitionResources,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
        ControlActionEventData {
            entity,
            control_action,
        }: ControlActionEventData,
        value: bool,
    ) {
        self.processed_entities.add(entity.id());

        if let (Some(character_control_transitions_handle), Some(controller_input)) = (
            character_control_transitions_handles.get(entity),
            controller_inputs.get(entity),
        ) {
            let character_control_transitions = character_control_transitions_assets
                .get(character_control_transitions_handle)
                .expect("Expected `CharacterControlTransitions` to be loaded.");

            let transition_sequence_id = character_control_transitions
                .iter()
                .filter_map(|character_control_transition| {
                    let control_transition = *character_control_transition.control_transition();
                    let control_transition_requirements =
                        &character_control_transition.control_transition_requirements;

                    match control_transition {
                        ControlTransition::ActionPress(ActionPress {
                            action,
                            sequence_id,
                        }) => {
                            if value && control_action == action {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        ControlTransition::ActionRelease(ActionRelease {
                            action,
                            sequence_id,
                        }) => {
                            if !value && control_action == action {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        ControlTransition::ActionHold(action_hold) => {
                            Self::hold_transition_action(action_hold, *controller_input)
                                .map(|transition| (transition, control_transition_requirements))
                        }
                        _ => None,
                    }
                })
                .filter_map(|(sequence_id, control_transition_requirements)| {
                    Self::process_transition(
                        charge_use_ec,
                        control_transition_requirement_system_data,
                        entity,
                        sequence_id,
                        &control_transition_requirements,
                    )
                })
                .next();

            if let Some(transition_sequence_id) = transition_sequence_id {
                character_sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `CharacterSequenceId` component.");
            }
        }
    }

    fn handle_axis_event(
        &mut self,
        CharacterControlTransitionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref character_control_transitions_handles,
            ref character_control_transitions_assets,
            ref mut character_sequence_ids,
            ref mut charge_use_ec,
        }: &mut CharacterControlTransitionsTransitionResources,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
        AxisMoveEventData {
            entity,
            axis: control_axis,
            value,
        }: AxisMoveEventData,
    ) {
        self.processed_entities.add(entity.id());

        if let (Some(character_control_transitions_handle), Some(controller_input)) = (
            character_control_transitions_handles.get(entity),
            controller_inputs.get(entity),
        ) {
            let character_control_transitions = character_control_transitions_assets
                .get(character_control_transitions_handle)
                .expect("Expected `CharacterControlTransitions` to be loaded.");

            let transition_sequence_id = character_control_transitions
                .iter()
                .filter_map(|character_control_transition| {
                    let control_transition = *character_control_transition.control_transition();
                    let control_transition_requirements =
                        &character_control_transition.control_transition_requirements;

                    match control_transition {
                        ControlTransition::AxisPress(AxisTransition { axis, sequence_id }) => {
                            if relative_ne!(0., value) && control_axis == axis {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        ControlTransition::AxisRelease(AxisTransition { axis, sequence_id }) => {
                            if relative_eq!(0., value) && control_axis == axis {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        ControlTransition::AxisHold(axis_hold) => {
                            Self::hold_transition_axis(axis_hold, *controller_input)
                                .map(|transition| (transition, control_transition_requirements))
                        }
                        _ => None,
                    }
                })
                .filter_map(|(sequence_id, control_transition_requirements)| {
                    Self::process_transition(
                        charge_use_ec,
                        control_transition_requirement_system_data,
                        entity,
                        sequence_id,
                        &control_transition_requirements,
                    )
                })
                .next();

            if let Some(transition_sequence_id) = transition_sequence_id {
                character_sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `CharacterSequenceId` component.");
            }
        }
    }

    /// Processes `CharacterControlTransitions` for entities without any `ControlInputEvent`.
    ///
    /// Checks the `ControllerInput` state for any `Hold` and `Fallback` transitions.
    fn process_hold_and_fallback_transitions(
        &self,
        CharacterControlTransitionsTransitionResources {
            ref entities,
            ref controller_inputs,
            ref character_control_transitions_handles,
            ref character_control_transitions_assets,
            ref mut character_sequence_ids,
            ref mut charge_use_ec,
        }: &mut CharacterControlTransitionsTransitionResources,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
    ) {
        (
            entities,
            character_control_transitions_handles,
            controller_inputs,
            !&self.processed_entities,
        )
            .join()
            .for_each(
                |(entity, character_control_transitions_handle, controller_input, _)| {
                    let character_control_transitions = character_control_transitions_assets
                        .get(character_control_transitions_handle)
                        .expect("Expected `CharacterControlTransitions` to be loaded.");

                    let transition_sequence_id = character_control_transitions
                        .iter()
                        .filter_map(|character_control_transition| {
                            let control_transition =
                                character_control_transition.control_transition();
                            let control_transition_requirements =
                                &character_control_transition.control_transition_requirements;

                            match control_transition {
                                ControlTransition::ActionHold(action_hold) => {
                                    Self::hold_transition_action(*action_hold, *controller_input)
                                        .map(|transition| {
                                            (transition, control_transition_requirements)
                                        })
                                }
                                ControlTransition::AxisHold(axis_hold) => {
                                    Self::hold_transition_axis(*axis_hold, *controller_input).map(
                                        |transition| (transition, control_transition_requirements),
                                    )
                                }
                                ControlTransition::Fallback(FallbackTransition { sequence_id }) => {
                                    Some((*sequence_id, control_transition_requirements))
                                }
                                _ => None,
                            }
                        })
                        .filter_map(|(sequence_id, control_transition_requirements)| {
                            Self::process_transition(
                                charge_use_ec,
                                control_transition_requirement_system_data,
                                entity,
                                sequence_id,
                                &control_transition_requirements,
                            )
                        })
                        .next();

                    if let Some(transition_sequence_id) = transition_sequence_id {
                        character_sequence_ids
                            .insert(entity, transition_sequence_id)
                            .expect("Failed to insert `CharacterSequenceId` component.");
                    }
                },
            );
    }

    /// Returns the transition sequence ID if the action button for that hold transition is held.
    ///
    /// # Parameters
    ///
    /// * `action_hold`: `ControlAction` and sequence ID the hold transition applies to.
    /// * `controller_input`: Controller input status.
    fn hold_transition_action(
        ActionHold {
            action,
            sequence_id,
        }: ActionHold<CharacterSequenceId>,
        controller_input: ControllerInput,
    ) -> Option<CharacterSequenceId> {
        match action {
            ControlAction::Defend => {
                if controller_input.defend {
                    Some(sequence_id)
                } else {
                    None
                }
            }
            ControlAction::Jump => {
                if controller_input.jump {
                    Some(sequence_id)
                } else {
                    None
                }
            }
            ControlAction::Attack => {
                if controller_input.attack {
                    Some(sequence_id)
                } else {
                    None
                }
            }
            ControlAction::Special => {
                if controller_input.special {
                    Some(sequence_id)
                } else {
                    None
                }
            }
        }
    } // kcov-ignore

    /// Returns the transition sequence ID if the axis input for that hold transition is valued.
    ///
    /// # Parameters
    ///
    /// * `axis_transition`: `Axis` and sequence ID the hold transition applies to.
    /// * `controller_input`: Controller input status.
    fn hold_transition_axis(
        AxisTransition { axis, sequence_id }: AxisTransition<CharacterSequenceId>,
        controller_input: ControllerInput,
    ) -> Option<CharacterSequenceId> {
        match axis {
            Axis::X => {
                if relative_ne!(0., controller_input.x_axis_value) {
                    Some(sequence_id)
                } else {
                    None
                }
            }
            Axis::Z => {
                if relative_ne!(0., controller_input.z_axis_value) {
                    Some(sequence_id)
                } else {
                    None
                }
            }
        }
    } // kcov-ignore

    fn process_transition(
        charge_use_ec: &mut EventChannel<ChargeUseEvent>,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
        entity: Entity,
        sequence_id: CharacterSequenceId,
        control_transition_requirements: &[ControlTransitionRequirement],
    ) -> Option<CharacterSequenceId> {
        if Self::transition_requirements_met(
            control_transition_requirement_system_data,
            &control_transition_requirements,
            entity,
        ) {
            control_transition_requirements
                .iter()
                .filter_map(|control_transition_requirement| {
                    if let ControlTransitionRequirement::Charge(charge_points) =
                        control_transition_requirement
                    {
                        Some(ChargeUseEvent {
                            entity,
                            charge_points: *charge_points,
                        })
                    } else {
                        None
                    }
                })
                .for_each(|charge_use_event| charge_use_ec.single_write(charge_use_event));
            Some(sequence_id)
        } else {
            None
        }
    }

    fn transition_requirements_met(
        ControlTransitionRequirementSystemData {
            health_pointses,
            skill_pointses,
            charge_tracker_clocks,
            controller_inputs,
            mirroreds,
        }: &ControlTransitionRequirementSystemData,
        control_transition_requirements: &[ControlTransitionRequirement],
        entity: Entity,
    ) -> bool {
        let (health_points, skill_points, charge_tracker_clock, controller_input, mirrored) = (
            health_pointses.get(entity).copied(),
            skill_pointses.get(entity).copied(),
            charge_tracker_clocks.get(entity).copied(),
            controller_inputs.get(entity).copied(),
            mirroreds.get(entity).copied(),
        );

        control_transition_requirements
            .iter()
            .all(|control_transition_requirement| {
                control_transition_requirement.is_met(
                    health_points,
                    skill_points,
                    charge_tracker_clock,
                    controller_input,
                    mirrored,
                )
            })
    }
}

impl<'s> System<'s> for CharacterControlTransitionsTransitionSystem {
    type SystemData = CharacterControlTransitionsTransitionSystemData<'s>;

    fn run(
        &mut self,
        CharacterControlTransitionsTransitionSystemData {
            control_input_ec,
            mut character_control_transitions_transition_resources,
            control_transition_requirement_system_data,
        }: Self::SystemData,
    ) {
        self.processed_entities.clear();

        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| match ev {
                ControlInputEvent::ControlActionPress(control_action_event_data) => {
                    self.handle_action_event(
                        &mut character_control_transitions_transition_resources,
                        &control_transition_requirement_system_data,
                        *control_action_event_data,
                        true,
                    );
                }
                ControlInputEvent::ControlActionRelease(control_action_event_data) => {
                    self.handle_action_event(
                        &mut character_control_transitions_transition_resources,
                        &control_transition_requirement_system_data,
                        *control_action_event_data,
                        false,
                    );
                }
                ControlInputEvent::AxisMoved(axis_move_event_data) => {
                    self.handle_axis_event(
                        &mut character_control_transitions_transition_resources,
                        &control_transition_requirement_system_data,
                        *axis_move_event_data,
                    );
                }
            });

        self.process_hold_and_fallback_transitions(
            &mut character_control_transitions_transition_resources,
            &control_transition_requirement_system_data,
        );
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);

        self.control_input_event_rid = Some(
            res.fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use std::{iter::FromIterator, path::PathBuf};

    use amethyst::{
        assets::{AssetStorage, Loader},
        ecs::{Builder, Entity, Read, ReadExpect, World, WriteStorage},
        shrev::{EventChannel, ReaderId},
        Error,
    };
    use application::resource::IoUtils;
    use application_test_support::AutexousiousApplication;
    use character_loading::{
        ControlTransitionsSequenceLoader, ControlTransitionsSequenceLoaderParams,
    };
    use character_model::{
        config::{CharacterSequence, CharacterSequenceId},
        loaded::{
            CharacterControlTransitions, CharacterControlTransitionsHandle,
            CharacterControlTransitionsSequence, CharacterControlTransitionsSequenceHandle,
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
    use shred_derive::SystemData;

    use super::CharacterControlTransitionsTransitionSystem;

    #[test]
    fn inserts_transition_for_action_press_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
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
                sequence_id: CharacterSequenceId::StandAttack0,
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_action_release_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
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
                sequence_id: CharacterSequenceId::DashBack,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: CharacterSequenceId::Flinch0,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
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
                sequence_id: CharacterSequenceId::Jump,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
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
                sequence_id: CharacterSequenceId::DashBack,
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_axis_press_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
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
                sequence_id: CharacterSequenceId::FallForwardAscend,
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_axis_release_event() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
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
                sequence_id: CharacterSequenceId::LieFaceDown,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: CharacterSequenceId::FallForwardDescend,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
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
                sequence_id: CharacterSequenceId::FallForwardAscend,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
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
                sequence_id: CharacterSequenceId::Dazed,
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn inserts_transition_for_fallback() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
                controller_input: ControllerInput::default(),
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: CharacterSequenceId::RunStop,
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
                sequence_id: CharacterSequenceId::Stand,
                controller_input: controller_input,
                control_input_event_fn: None,
                charge_tracker_clock: ChargeTrackerClock::new_with_value(100, 100),
            },
            ExpectedParams {
                sequence_id: CharacterSequenceId::Stand,
                charge_use_events_fn: None,
            },
        )
    }

    #[test]
    fn sends_charge_use_event_when_requirement_met() -> Result<(), Error> {
        run_test(
            SetupParams {
                sequence_id: CharacterSequenceId::Stand,
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
                sequence_id: CharacterSequenceId::DashBack,
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
                sequence_id: CharacterSequenceId::Stand,
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
                sequence_id: CharacterSequenceId::Stand,
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
            .with_system(CharacterControlTransitionsTransitionSystem::new(), "", &[])
            .with_setup(register_reader)
            .with_setup(move |world| {
                let character_control_transitions_sequence_handle = {
                    let (
                        loader,
                        character_control_transitions_assets,
                        character_control_transitions_sequence_assets,
                    ) = world.system_data::<(
                        ReadExpect<'_, Loader>,
                        Read<'_, AssetStorage<CharacterControlTransitions>>,
                        Read<'_, AssetStorage<CharacterControlTransitionsSequence>>,
                    )>();

                    let control_transitions_sequence_loader_params =
                        ControlTransitionsSequenceLoaderParams {
                            loader: &loader,
                            character_control_transitions_assets:
                                &character_control_transitions_assets,
                            character_control_transitions_sequence_assets:
                                &character_control_transitions_sequence_assets,
                        };
                    let test_character_sequence = test_character_sequence();

                    ControlTransitionsSequenceLoader::load(
                        &control_transitions_sequence_loader_params,
                        None,
                        &test_character_sequence,
                    )
                };

                world.add_resource(character_control_transitions_sequence_handle);
            })
            // Allow `AssetStorage`s to process loaded data.
            .with_setup(move |world| {
                let character_control_transitions_handle = {
                    let character_control_transitions_sequence_assets = world.system_data::<Read<
                        '_,
                        AssetStorage<CharacterControlTransitionsSequence>,
                    >>(
                    );

                    let character_control_transitions_sequence_handle = world
                        .read_resource::<CharacterControlTransitionsSequenceHandle>()
                        .clone();
                    let character_control_transitions_sequence =
                        character_control_transitions_sequence_assets
                            .get(&character_control_transitions_sequence_handle)
                            .expect(
                                "Expected `character_control_transitions_sequence` to be loaded.",
                            );
                    character_control_transitions_sequence
                        .first()
                        .expect(
                            "Expected `character_control_transitions_sequence` to contain one \
                             `character_control_transitions_handle`.",
                        )
                        .clone()
                };

                let entity = world.create_entity().build();
                {
                    let TestSystemData {
                        mut character_sequence_ids,
                        mut character_control_transitions_handles,
                        mut health_pointses,
                        mut skill_pointses,
                        mut charge_tracker_clocks,
                        mut mirroreds,
                        mut controller_inputs,
                    } = world.system_data::<TestSystemData>();

                    character_sequence_ids
                        .insert(entity, sequence_id_setup)
                        .expect("Failed to insert `CharacterSequenceId` component.");
                    character_control_transitions_handles
                        .insert(entity, character_control_transitions_handle)
                        .expect("Failed to insert `CharacterControlTransitionsHandle` component.");
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

                world.add_resource(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();

                let character_sequence_id = {
                    let character_sequence_ids = world.read_storage::<CharacterSequenceId>();

                    character_sequence_ids
                        .get(entity)
                        .copied()
                        .expect("Expected `CharacterSequenceId` component to exist.")
                };

                assert_eq!(sequence_id_expected, character_sequence_id);

                if let Some(charge_use_events_fn) = charge_use_events_fn {
                    let charge_use_events = charge_use_events_fn(entity);

                    expect_events(world, charge_use_events);
                }
            })
            .run_isolated()
    }

    fn test_character_sequence() -> CharacterSequence {
        let test_character_sequence_toml = "test_character_sequence.toml";
        let test_character_sequence_path = PathBuf::from_iter(&[
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            test_character_sequence_toml,
        ]);
        let contents = IoUtils::read_file(&test_character_sequence_path).unwrap_or_else(|e| {
            panic!(
                "Failed to read `{}`. Error: {}",
                test_character_sequence_toml, e
            )
        });

        toml::from_slice::<CharacterSequence>(&contents)
            .expect("Failed to load `test_character_sequence.toml`.")
    }

    fn register_reader(world: &mut World) {
        let reader_id = {
            let mut ec = world.write_resource::<EventChannel<ChargeUseEvent>>();
            ec.register_reader()
        }; // kcov-ignore
        world.add_resource(reader_id);
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
        character_sequence_ids: WriteStorage<'s, CharacterSequenceId>,
        #[derivative(Debug = "ignore")]
        character_control_transitions_handles: WriteStorage<'s, CharacterControlTransitionsHandle>,
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
        sequence_id: CharacterSequenceId,
        controller_input: ControllerInput,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
        charge_tracker_clock: ChargeTrackerClock,
    }

    struct ExpectedParams {
        sequence_id: CharacterSequenceId,
        charge_use_events_fn: Option<fn(Entity) -> Vec<ChargeUseEvent>>,
    }
}
