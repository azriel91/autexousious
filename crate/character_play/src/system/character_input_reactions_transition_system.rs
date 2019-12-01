use amethyst::{
    assets::AssetStorage,
    ecs::{BitSet, Entities, Entity, Join, Read, ReadStorage, System, World, Write, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use approx::{relative_eq, relative_ne};
use character_model::{
    config::{ControlTransitionRequirement, ControlTransitionRequirementParams},
    loaded::{CharacterInputReactions, CharacterInputReactionsHandle},
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
    ActionHold, ActionPress, ActionRelease, AxisTransition, ControlTransitionLike,
    FallbackTransition, InputReaction, SequenceId,
};

use crate::ControlTransitionRequirementSystemData;

/// Updates `SequenceId` based on `ControlInputEvent`s and held buttons.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterInputReactionsTransitionSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
    /// Pre-allocated bitset to track entities whose transitions have already been checked.
    #[new(default)]
    processed_entities: BitSet,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterInputReactionsTransitionSystemData<'s> {
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `CharacterInputReactionsTransitionResources`.
    pub character_input_reactions_transition_resources:
        CharacterInputReactionsTransitionResources<'s>,
    /// `ControlTransitionRequirementSystemData`.
    pub control_transition_requirement_system_data: ControlTransitionRequirementSystemData<'s>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterInputReactionsTransitionResources<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `CharacterInputReactionsHandle` components.
    #[derivative(Debug = "ignore")]
    pub character_input_reactions_handles: ReadStorage<'s, CharacterInputReactionsHandle>,
    /// `CharacterInputReactions` assets.
    #[derivative(Debug = "ignore")]
    pub character_input_reactions_assets: Read<'s, AssetStorage<CharacterInputReactions>>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `ChargeUseEvent` channel.
    #[derivative(Debug = "ignore")]
    pub charge_use_ec: Write<'s, EventChannel<ChargeUseEvent>>,
}

impl CharacterInputReactionsTransitionSystem {
    fn handle_action_event(
        &mut self,
        CharacterInputReactionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref character_input_reactions_handles,
            ref character_input_reactions_assets,
            ref mut sequence_ids,
            ref mut charge_use_ec,
        }: &mut CharacterInputReactionsTransitionResources,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
        ControlActionEventData {
            entity,
            control_action,
        }: ControlActionEventData,
        value: bool,
    ) {
        self.processed_entities.add(entity.id());

        if let (Some(character_input_reactions_handle), Some(controller_input)) = (
            character_input_reactions_handles.get(entity),
            controller_inputs.get(entity),
        ) {
            let character_input_reactions = character_input_reactions_assets
                .get(character_input_reactions_handle)
                .expect("Expected `CharacterInputReactions` to be loaded.");

            let transition_sequence_id = character_input_reactions
                .iter()
                .filter_map(|character_control_transition| {
                    let input_reaction = *character_control_transition.input_reaction();
                    let control_transition_requirements =
                        &character_control_transition.control_transition_requirements;

                    match input_reaction {
                        InputReaction::ActionPress(ActionPress {
                            action,
                            sequence_id,
                        }) => {
                            if value && control_action == action {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        InputReaction::ActionRelease(ActionRelease {
                            action,
                            sequence_id,
                        }) => {
                            if !value && control_action == action {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        InputReaction::ActionHold(action_hold) => {
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
                sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `SequenceId` component.");
            }
        }
    }

    fn handle_axis_event(
        &mut self,
        CharacterInputReactionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref character_input_reactions_handles,
            ref character_input_reactions_assets,
            ref mut sequence_ids,
            ref mut charge_use_ec,
        }: &mut CharacterInputReactionsTransitionResources,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
        AxisMoveEventData {
            entity,
            axis: control_axis,
            value,
        }: AxisMoveEventData,
    ) {
        self.processed_entities.add(entity.id());

        if let (Some(character_input_reactions_handle), Some(controller_input)) = (
            character_input_reactions_handles.get(entity),
            controller_inputs.get(entity),
        ) {
            let character_input_reactions = character_input_reactions_assets
                .get(character_input_reactions_handle)
                .expect("Expected `CharacterInputReactions` to be loaded.");

            let transition_sequence_id = character_input_reactions
                .iter()
                .filter_map(|character_control_transition| {
                    let input_reaction = *character_control_transition.input_reaction();
                    let control_transition_requirements =
                        &character_control_transition.control_transition_requirements;

                    match input_reaction {
                        InputReaction::AxisPress(AxisTransition { axis, sequence_id }) => {
                            if relative_ne!(0., value) && control_axis == axis {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        InputReaction::AxisRelease(AxisTransition { axis, sequence_id }) => {
                            if relative_eq!(0., value) && control_axis == axis {
                                Some((sequence_id, control_transition_requirements))
                            } else {
                                None
                            }
                        }
                        InputReaction::AxisHold(axis_hold) => {
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
                sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `SequenceId` component.");
            }
        }
    }

    /// Processes `CharacterInputReactions` for entities without any `ControlInputEvent`.
    ///
    /// Checks the `ControllerInput` state for any `Hold` and `Fallback` transitions.
    fn process_hold_and_fallback_transitions(
        &self,
        CharacterInputReactionsTransitionResources {
            ref entities,
            ref controller_inputs,
            ref character_input_reactions_handles,
            ref character_input_reactions_assets,
            ref mut sequence_ids,
            ref mut charge_use_ec,
        }: &mut CharacterInputReactionsTransitionResources,
        control_transition_requirement_system_data: &ControlTransitionRequirementSystemData,
    ) {
        (
            entities,
            character_input_reactions_handles,
            controller_inputs,
            !&self.processed_entities,
        )
            .join()
            .for_each(
                |(entity, character_input_reactions_handle, controller_input, _)| {
                    let character_input_reactions = character_input_reactions_assets
                        .get(character_input_reactions_handle)
                        .expect("Expected `CharacterInputReactions` to be loaded.");

                    let transition_sequence_id = character_input_reactions
                        .iter()
                        .filter_map(|character_control_transition| {
                            let input_reaction = character_control_transition.input_reaction();
                            let control_transition_requirements =
                                &character_control_transition.control_transition_requirements;

                            match input_reaction {
                                InputReaction::ActionHold(action_hold) => {
                                    Self::hold_transition_action(*action_hold, *controller_input)
                                        .map(|transition| {
                                            (transition, control_transition_requirements)
                                        })
                                }
                                InputReaction::AxisHold(axis_hold) => {
                                    Self::hold_transition_axis(*axis_hold, *controller_input).map(
                                        |transition| (transition, control_transition_requirements),
                                    )
                                }
                                InputReaction::Fallback(FallbackTransition { sequence_id }) => {
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
                        sequence_ids
                            .insert(entity, transition_sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
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
        }: ActionHold,
        controller_input: ControllerInput,
    ) -> Option<SequenceId> {
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
        AxisTransition { axis, sequence_id }: AxisTransition,
        controller_input: ControllerInput,
    ) -> Option<SequenceId> {
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
        sequence_id: SequenceId,
        control_transition_requirements: &[ControlTransitionRequirement],
    ) -> Option<SequenceId> {
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
            charge_use_modes,
            controller_inputs,
            mirroreds,
        }: &ControlTransitionRequirementSystemData,
        control_transition_requirements: &[ControlTransitionRequirement],
        entity: Entity,
    ) -> bool {
        let (
            health_points,
            skill_points,
            charge_tracker_clock,
            charge_use_mode,
            controller_input,
            mirrored,
        ) = (
            health_pointses.get(entity).copied(),
            skill_pointses.get(entity).copied(),
            charge_tracker_clocks.get(entity).copied(),
            charge_use_modes.get(entity).copied(),
            controller_inputs.get(entity).copied(),
            mirroreds.get(entity).copied(),
        );

        let control_transition_requirement_params = ControlTransitionRequirementParams {
            health_points,
            skill_points,
            charge_tracker_clock,
            charge_use_mode,
            controller_input,
            mirrored,
        };

        control_transition_requirements
            .iter()
            .all(|control_transition_requirement| {
                control_transition_requirement.is_met(control_transition_requirement_params)
            })
    }
}

impl<'s> System<'s> for CharacterInputReactionsTransitionSystem {
    type SystemData = CharacterInputReactionsTransitionSystemData<'s>;

    fn run(
        &mut self,
        CharacterInputReactionsTransitionSystemData {
            control_input_ec,
            mut character_input_reactions_transition_resources,
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
                        &mut character_input_reactions_transition_resources,
                        &control_transition_requirement_system_data,
                        *control_action_event_data,
                        true,
                    );
                }
                ControlInputEvent::ControlActionRelease(control_action_event_data) => {
                    self.handle_action_event(
                        &mut character_input_reactions_transition_resources,
                        &control_transition_requirement_system_data,
                        *control_action_event_data,
                        false,
                    );
                }
                ControlInputEvent::AxisMoved(axis_move_event_data) => {
                    self.handle_axis_event(
                        &mut character_input_reactions_transition_resources,
                        &control_transition_requirement_system_data,
                        *axis_move_event_data,
                    );
                }
            });

        self.process_hold_and_fallback_transitions(
            &mut character_input_reactions_transition_resources,
            &control_transition_requirement_system_data,
        );
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.control_input_event_rid = Some(
            world
                .fetch_mut::<EventChannel<ControlInputEvent>>()
                .register_reader(),
        );
    }
}
