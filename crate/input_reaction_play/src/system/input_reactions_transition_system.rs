use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    assets::AssetStorage,
    ecs::{BitSet, Entities, Entity, Join, Read, ReadStorage, System, World, WriteStorage},
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use approx::{relative_eq, relative_ne};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::{Axis, ControlAction},
    play::{
        AxisMoveEventData, ControlActionEventData, ControlInputEvent, ControllerInput,
        InputControlled,
    },
};
use input_reaction_model::{
    config::{InputReactionAppEvents, InputReactionRequirement},
    loaded::{
        AxisTransition, FallbackTransition, InputReaction, InputReactions, InputReactionsHandle,
        ReactionEffect, ReactionEffectData,
    },
};
use sequence_model::loaded::SequenceId;

use crate::{IrAppEventSender, IrAppEventSenderSystemData};

/// Updates `SequenceId` based on `ControlInputEvent`s and held buttons.
///
/// # Type Parameters
///
/// * `IRR`: `InputReactionRequirement`.
#[derive(Debug, Default, new)]
pub struct InputReactionsTransitionSystem<IRR> {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
    /// Pre-allocated bitset to track entities whose transitions have already
    /// been checked.
    #[new(default)]
    processed_entities: BitSet,
    /// Marker.
    marker: PhantomData<IRR>,
}

/// `InputReactionsTransitionSystemData`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputReactionsTransitionSystemData<'s, IRR>
where
    IRR: InputReactionRequirement<'s> + Send + Sync + 'static,
    IRR::SystemData: Debug,
{
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Read<'s, EventChannel<ControlInputEvent>>,
    /// `InputReactionsTransitionResources`.
    pub input_reactions_transition_resources: InputReactionsTransitionResources<'s, IRR>,
    /// `InputReactionRequirement` system data.
    pub requirement_system_data: IRR::SystemData,
}

/// `InputReactionsTransitionResources`.
#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputReactionsTransitionResources<'s, IRR>
where
    IRR: Send + Sync + 'static,
{
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ControllerInput` components.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `InputReactionsHandle` components.
    #[derivative(Debug = "ignore")]
    pub input_reactions_handles: ReadStorage<'s, InputReactionsHandle<InputReaction<IRR>>>,
    /// `InputReactions` assets.
    #[derivative(Debug = "ignore")]
    pub input_reactions_assets: Read<'s, AssetStorage<InputReactions<InputReaction<IRR>>>>,
    /// `SequenceId` components.
    #[derivative(Debug = "ignore")]
    pub sequence_ids: WriteStorage<'s, SequenceId>,
    /// `IrAppEventSenderSystemData`.
    #[derivative(Debug = "ignore")]
    pub ir_app_event_sender_system_data: IrAppEventSenderSystemData<'s>,
}

impl<'s, IRR> InputReactionsTransitionSystem<IRR>
where
    IRR: InputReactionRequirement<'s> + Send + Sync + 'static,
{
    fn handle_action_event(
        &mut self,
        InputReactionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref input_reactions_handles,
            ref input_reactions_assets,
            ref mut sequence_ids,
            ref mut ir_app_event_sender_system_data,
            ..
        }: &mut InputReactionsTransitionResources<IRR>,
        requirement_system_data: &mut IRR::SystemData,
        ControlActionEventData {
            controller_id,
            entity,
            control_action,
        }: ControlActionEventData,
        value: bool,
    ) {
        self.processed_entities.add(entity.id());

        if let (Some(input_reactions_handle), Some(controller_input)) = (
            input_reactions_handles.get(entity),
            controller_inputs.get(entity),
        ) {
            let input_reactions = input_reactions_assets
                .get(input_reactions_handle)
                .expect("Expected `InputReactions` to be loaded.");

            let transition_sequence_id = input_reactions
                .iter()
                .filter_map(|input_reaction| {
                    let input_reaction_requirement = &input_reaction.requirement;

                    match &input_reaction.effect {
                        ReactionEffect::ActionPress(ReactionEffectData {
                            action,
                            sequence_id,
                            events,
                        }) => {
                            if value && control_action == *action {
                                Some((*sequence_id, events, input_reaction_requirement))
                            } else {
                                None
                            }
                        }
                        ReactionEffect::ActionRelease(ReactionEffectData {
                            action,
                            sequence_id,
                            events,
                        }) => {
                            if !value && control_action == *action {
                                Some((*sequence_id, events, input_reaction_requirement))
                            } else {
                                None
                            }
                        }
                        ReactionEffect::ActionHold(reaction_effect_data) => {
                            Self::hold_transition_action(reaction_effect_data, *controller_input)
                                .map(|(transition, events)| {
                                    (transition, events, input_reaction_requirement)
                                })
                        }
                        _ => None,
                    }
                })
                .find_map(|(sequence_id, events, input_reaction_requirement)| {
                    Self::process_transition(
                        requirement_system_data,
                        entity,
                        sequence_id,
                        events,
                        input_reaction_requirement,
                    )
                });

            if let Some((transition_sequence_id, events)) = transition_sequence_id {
                events.iter().copied().for_each(|event| {
                    IrAppEventSender::send(
                        ir_app_event_sender_system_data,
                        Some(controller_id),
                        entity,
                        event,
                    )
                });

                sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `SequenceId` component.");
            }
        }
    }

    fn handle_axis_event(
        &mut self,
        InputReactionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref input_reactions_handles,
            ref input_reactions_assets,
            ref mut sequence_ids,
            ref mut ir_app_event_sender_system_data,
            ..
        }: &mut InputReactionsTransitionResources<IRR>,
        requirement_system_data: &mut IRR::SystemData,
        AxisMoveEventData {
            controller_id,
            entity,
            axis: control_axis,
            value,
        }: AxisMoveEventData,
    ) {
        self.processed_entities.add(entity.id());

        if let (Some(input_reactions_handle), Some(controller_input)) = (
            input_reactions_handles.get(entity),
            controller_inputs.get(entity),
        ) {
            let input_reactions = input_reactions_assets
                .get(input_reactions_handle)
                .expect("Expected `InputReactions` to be loaded.");

            let transition_sequence_id = input_reactions
                .iter()
                .filter_map(|input_reaction| {
                    let input_reaction_requirement = &input_reaction.requirement;

                    match &input_reaction.effect {
                        ReactionEffect::AxisPress(AxisTransition {
                            axis,
                            sequence_id,
                            events,
                        }) => {
                            if relative_ne!(0., value) && control_axis == *axis {
                                Some((*sequence_id, events, input_reaction_requirement))
                            } else {
                                None
                            }
                        }
                        ReactionEffect::AxisRelease(AxisTransition {
                            axis,
                            sequence_id,
                            events,
                        }) => {
                            if relative_eq!(0., value) && control_axis == *axis {
                                Some((*sequence_id, events, input_reaction_requirement))
                            } else {
                                None
                            }
                        }
                        ReactionEffect::AxisHold(reaction_effect_data) => {
                            Self::hold_transition_axis(reaction_effect_data, *controller_input).map(
                                |(transition, events)| {
                                    (transition, events, input_reaction_requirement)
                                },
                            )
                        }
                        _ => None,
                    }
                })
                .find_map(|(sequence_id, events, input_reaction_requirement)| {
                    Self::process_transition(
                        requirement_system_data,
                        entity,
                        sequence_id,
                        events,
                        input_reaction_requirement,
                    )
                });

            if let Some((transition_sequence_id, events)) = transition_sequence_id {
                events.iter().copied().for_each(|event| {
                    IrAppEventSender::send(
                        ir_app_event_sender_system_data,
                        Some(controller_id),
                        entity,
                        event,
                    )
                });

                sequence_ids
                    .insert(entity, transition_sequence_id)
                    .expect("Failed to insert `SequenceId` component.");
            }
        }
    }

    /// Processes `InputReactions` for entities without any `ControlInputEvent`.
    ///
    /// Checks the `ControllerInput` state for any `Hold` and `Fallback`
    /// transitions.
    fn process_hold_and_fallback_transitions(
        &self,
        InputReactionsTransitionResources {
            ref entities,
            ref controller_inputs,
            ref input_controlleds,
            ref input_reactions_handles,
            ref input_reactions_assets,
            ref mut sequence_ids,
            ref mut ir_app_event_sender_system_data,
        }: &mut InputReactionsTransitionResources<IRR>,
        requirement_system_data: &mut IRR::SystemData,
    ) {
        (
            entities,
            input_reactions_handles,
            controller_inputs.maybe(),
            input_controlleds.maybe(),
            !&self.processed_entities,
        )
            .join()
            .for_each(
                |(entity, input_reactions_handle, controller_input, input_controlled, _)| {
                    let input_reactions = input_reactions_assets
                        .get(input_reactions_handle)
                        .expect("Expected `InputReactions` to be loaded.");

                    let transition_sequence_id = input_reactions
                        .iter()
                        .filter_map(|input_reaction| {
                            let input_reaction_requirement = &input_reaction.requirement;

                            match &input_reaction.effect {
                                ReactionEffect::ActionHold(reaction_effect_data) => {
                                    controller_input.and_then(|controller_input| {
                                        Self::hold_transition_action(
                                            reaction_effect_data,
                                            *controller_input,
                                        )
                                        .map(
                                            |(transition, events)| {
                                                (transition, events, input_reaction_requirement)
                                            },
                                        )
                                    })
                                }
                                ReactionEffect::AxisHold(reaction_effect_data) => controller_input
                                    .and_then(|controller_input| {
                                        Self::hold_transition_axis(
                                            reaction_effect_data,
                                            *controller_input,
                                        )
                                        .map(
                                            |(transition, events)| {
                                                (transition, events, input_reaction_requirement)
                                            },
                                        )
                                    }),
                                ReactionEffect::Fallback(FallbackTransition {
                                    sequence_id,
                                    events,
                                }) => Some((*sequence_id, events, input_reaction_requirement)),
                                _ => None,
                            }
                        })
                        .find_map(|(sequence_id, events, input_reaction_requirement)| {
                            Self::process_transition(
                                requirement_system_data,
                                entity,
                                sequence_id,
                                events,
                                input_reaction_requirement,
                            )
                        });

                    if let Some((transition_sequence_id, events)) = transition_sequence_id {
                        events.iter().copied().for_each(|event| {
                            IrAppEventSender::send(
                                ir_app_event_sender_system_data,
                                input_controlled
                                    .map(|input_controlled| input_controlled.controller_id),
                                entity,
                                event,
                            )
                        });

                        sequence_ids
                            .insert(entity, transition_sequence_id)
                            .expect("Failed to insert `SequenceId` component.");
                    }
                },
            );
    }

    /// Returns the transition sequence ID if the action button for that hold
    /// transition is held.
    ///
    /// # Parameters
    ///
    /// * `reaction_effect_data`: `ControlAction` and sequence ID the hold
    ///   transition applies to.
    /// * `controller_input`: Controller input status.
    fn hold_transition_action(
        ReactionEffectData {
            action,
            sequence_id,
            events,
        }: &ReactionEffectData,
        controller_input: ControllerInput,
    ) -> Option<(SequenceId, &InputReactionAppEvents)> {
        match action {
            ControlAction::Defend => {
                if controller_input.defend {
                    Some((*sequence_id, events))
                } else {
                    None
                }
            }
            ControlAction::Jump => {
                if controller_input.jump {
                    Some((*sequence_id, events))
                } else {
                    None
                }
            }
            ControlAction::Attack => {
                if controller_input.attack {
                    Some((*sequence_id, events))
                } else {
                    None
                }
            }
            ControlAction::Special => {
                if controller_input.special {
                    Some((*sequence_id, events))
                } else {
                    None
                }
            }
        }
    }

    // kcov-ignore

    /// Returns the transition sequence ID if the axis input for that hold
    /// transition is valued.
    ///
    /// # Parameters
    ///
    /// * `axis_transition`: `Axis` and sequence ID the hold transition applies
    ///   to.
    /// * `controller_input`: Controller input status.
    fn hold_transition_axis(
        AxisTransition {
            axis,
            sequence_id,
            events,
        }: &AxisTransition,
        controller_input: ControllerInput,
    ) -> Option<(SequenceId, &InputReactionAppEvents)> {
        match axis {
            Axis::X => {
                if relative_ne!(0., controller_input.x_axis_value) {
                    Some((*sequence_id, events))
                } else {
                    None
                }
            }
            Axis::Z => {
                if relative_ne!(0., controller_input.z_axis_value) {
                    Some((*sequence_id, events))
                } else {
                    None
                }
            }
        }
    }

    // kcov-ignore

    fn process_transition<'f>(
        requirement_system_data: &mut IRR::SystemData,
        entity: Entity,
        sequence_id: SequenceId,
        events: &'f InputReactionAppEvents,
        input_reaction_requirement: &IRR,
    ) -> Option<(SequenceId, &'f InputReactionAppEvents)> {
        if input_reaction_requirement.requirement_met(requirement_system_data, entity) {
            Some((sequence_id, events))
        } else {
            None
        }
    }
}

impl<'s, IRR> System<'s> for InputReactionsTransitionSystem<IRR>
where
    IRR: InputReactionRequirement<'s> + Send + Sync + 'static,
    IRR::SystemData: Debug,
{
    type SystemData = InputReactionsTransitionSystemData<'s, IRR>;

    fn run(
        &mut self,
        InputReactionsTransitionSystemData {
            control_input_ec,
            mut input_reactions_transition_resources,
            mut requirement_system_data,
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
                        &mut input_reactions_transition_resources,
                        &mut requirement_system_data,
                        *control_action_event_data,
                        true,
                    );
                }
                ControlInputEvent::ControlActionRelease(control_action_event_data) => {
                    self.handle_action_event(
                        &mut input_reactions_transition_resources,
                        &mut requirement_system_data,
                        *control_action_event_data,
                        false,
                    );
                }
                ControlInputEvent::AxisMoved(axis_move_event_data) => {
                    self.handle_axis_event(
                        &mut input_reactions_transition_resources,
                        &mut requirement_system_data,
                        *axis_move_event_data,
                    );
                }
            });

        self.process_hold_and_fallback_transitions(
            &mut input_reactions_transition_resources,
            &mut requirement_system_data,
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
