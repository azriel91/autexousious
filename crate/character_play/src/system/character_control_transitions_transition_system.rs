use amethyst::{
    assets::AssetStorage,
    ecs::{BitSet, Entities, Join, Read, ReadStorage, Resources, System, SystemData, WriteStorage},
    shrev::{EventChannel, ReaderId},
};
use character_model::{
    config::CharacterSequenceId,
    loaded::{CharacterControlTransitions, CharacterControlTransitionsHandle},
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::{
    loaded::{
        ControlTransition, ControlTransitionHold, ControlTransitionLike, ControlTransitionPress,
        ControlTransitionRelease,
    },
    play::SequenceStatus,
};
use shred_derive::SystemData;

/// Updates `ControllerInput` based on input events.
#[derive(Debug, Default, NamedType, new)]
pub struct CharacterControlTransitionsTransitionSystem {
    /// Reader ID for the `ControlInputEvent` channel.
    #[new(default)]
    control_input_event_rid: Option<ReaderId<ControlInputEvent>>,
    /// Pre-allocated bitset to track entities whose transitions have already been checked.
    #[new(default)]
    processed_entities: BitSet,
}

type CharacterControlTransitionsTransitionSystemData<'s> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    CharacterControlTransitionsTransitionResources<'s>,
);

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct CharacterControlTransitionsTransitionResources<'s> {
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `ControllerInput` component storage.
    #[derivative(Debug = "ignore")]
    pub controller_inputs: ReadStorage<'s, ControllerInput>,
    /// `CharacterControlTransitionsHandle` component storage.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_handles: ReadStorage<'s, CharacterControlTransitionsHandle>,
    /// `CharacterControlTransitions` assets.
    #[derivative(Debug = "ignore")]
    pub character_control_transitions_assets: Read<'s, AssetStorage<CharacterControlTransitions>>,
    /// `CharacterSequenceId` component storage.
    #[derivative(Debug = "ignore")]
    pub character_sequence_ids: WriteStorage<'s, CharacterSequenceId>,
    /// `SequenceStatus` component storage.
    #[derivative(Debug = "ignore")]
    pub sequence_statuses: WriteStorage<'s, SequenceStatus>,
}

impl CharacterControlTransitionsTransitionSystem {
    fn handle_event(
        &mut self,
        CharacterControlTransitionsTransitionResources {
            entities: ref _entities,
            ref controller_inputs,
            ref character_control_transitions_handles,
            ref character_control_transitions_assets,
            ref mut character_sequence_ids,
            ref mut sequence_statuses,
        }: &mut CharacterControlTransitionsTransitionResources,
        ControlActionEventData {
            entity,
            control_action,
            value,
        }: ControlActionEventData,
    ) {
        self.processed_entities.add(entity.id());

        if let Some(character_control_transitions_handle) =
            character_control_transitions_handles.get(entity)
        {
            let character_control_transitions = character_control_transitions_assets
                .get(character_control_transitions_handle)
                .expect("Expected `CharacterControlTransitions` to be loaded.");

            let transition_sequence_id = character_control_transitions
                .iter()
                .filter_map(|character_control_transition| {
                    let control_transition = character_control_transition.control_transition();
                    let control_transition_requirement =
                        &character_control_transition.control_transition_requirement;

                    match control_transition {
                        ControlTransition::Press(ControlTransitionPress {
                            action,
                            sequence_id,
                        }) => {
                            if value {
                                if control_action == *action {
                                    return Some((sequence_id, control_transition_requirement));
                                }
                            }
                        }
                        ControlTransition::Release(ControlTransitionRelease {
                            action,
                            sequence_id,
                        }) => {
                            if !value {
                                if control_action == *action {
                                    return Some((sequence_id, control_transition_requirement));
                                }
                            }
                        }
                        ControlTransition::Hold(ControlTransitionHold {
                            action,
                            sequence_id,
                        }) => {
                            // Handle the held buttons
                            let controller_input = controller_inputs
                                .get(entity)
                                .expect("Expected `ControllerInput` to exist.");

                            match action {
                                ControlAction::Defend => {
                                    if controller_input.defend {
                                        return Some((sequence_id, control_transition_requirement));
                                    }
                                }
                                ControlAction::Jump => {
                                    if controller_input.jump {
                                        return Some((sequence_id, control_transition_requirement));
                                    }
                                }
                                ControlAction::Attack => {
                                    if controller_input.attack {
                                        return Some((sequence_id, control_transition_requirement));
                                    }
                                }
                                ControlAction::Special => {
                                    if controller_input.special {
                                        return Some((sequence_id, control_transition_requirement));
                                    }
                                }
                            }
                        }
                    };

                    None
                })
                .filter_map(|(sequence_id, _control_transition_requirement)| {
                    // TODO: Check if character meets requirement.
                    Some(sequence_id)
                })
                .next();

            if let Some(transition_sequence_id) = transition_sequence_id {
                character_sequence_ids
                    .insert(entity, *transition_sequence_id)
                    .expect("Failed to insert `CharacterSequenceId` component.");
                sequence_statuses
                    .insert(entity, SequenceStatus::Begin)
                    .expect("Failed to insert `SequenceStatus` component.");
            }
        }
    }

    /// Processes `CharacterControlTransitions` for entities without any `ControlInputEvent`.
    ///
    /// Checks the `ControllerInput` state for any `Hold` transitions.
    fn process_control_transition_holds(
        &self,
        CharacterControlTransitionsTransitionResources {
            ref entities,
            ref controller_inputs,
            ref character_control_transitions_handles,
            ref character_control_transitions_assets,
            ref mut character_sequence_ids,
            ref mut sequence_statuses,
        }: &mut CharacterControlTransitionsTransitionResources,
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
                            let control_transition_requirement =
                                &character_control_transition.control_transition_requirement;

                            if let ControlTransition::Hold(ControlTransitionHold {
                                action,
                                sequence_id,
                            }) = control_transition
                            {
                                // Handle the held buttons
                                match action {
                                    ControlAction::Defend => {
                                        if controller_input.defend {
                                            return Some((
                                                sequence_id,
                                                control_transition_requirement,
                                            ));
                                        }
                                    }
                                    ControlAction::Jump => {
                                        if controller_input.jump {
                                            return Some((
                                                sequence_id,
                                                control_transition_requirement,
                                            ));
                                        }
                                    }
                                    ControlAction::Attack => {
                                        if controller_input.attack {
                                            return Some((
                                                sequence_id,
                                                control_transition_requirement,
                                            ));
                                        }
                                    }
                                    ControlAction::Special => {
                                        if controller_input.special {
                                            return Some((
                                                sequence_id,
                                                control_transition_requirement,
                                            ));
                                        }
                                    }
                                }
                            }

                            None
                        })
                        .filter_map(|(sequence_id, _control_transition_requirement)| {
                            // TODO: Check if character meets requirement.
                            Some(sequence_id)
                        })
                        .next();

                    if let Some(transition_sequence_id) = transition_sequence_id {
                        character_sequence_ids
                            .insert(entity, *transition_sequence_id)
                            .expect("Failed to insert `CharacterSequenceId` component.");
                        sequence_statuses
                            .insert(entity, SequenceStatus::Begin)
                            .expect("Failed to insert `SequenceStatus` component.");
                    }
                },
            );
    }
}

impl<'s> System<'s> for CharacterControlTransitionsTransitionSystem {
    type SystemData = CharacterControlTransitionsTransitionSystemData<'s>;

    fn run(
        &mut self,
        (control_input_ec, mut character_control_transitions_transition_resources): Self::SystemData,
    ) {
        self.processed_entities.clear();

        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| {
                if let ControlInputEvent::ControlAction(control_action_event_data) = ev {
                    self.handle_event(
                        &mut character_control_transitions_transition_resources,
                        *control_action_event_data,
                    );
                }
            });

        self.process_control_transition_holds(
            &mut character_control_transitions_transition_resources,
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
