use amethyst::{
    assets::AssetStorage,
    ecs::{
        BitSet, Entities, Entity, Join, Read, ReadStorage, Resources, System, SystemData,
        WriteStorage,
    },
    shrev::{EventChannel, ReaderId},
};
use character_model::{
    config::{CharacterSequenceId, ControlTransitionRequirement},
    loaded::{CharacterControlTransitions, CharacterControlTransitionsHandle},
};
use derivative::Derivative;
use derive_new::new;
use game_input::ControllerInput;
use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
use named_type::NamedType;
use named_type_derive::NamedType;
use sequence_model::loaded::{
    ControlTransition, ControlTransitionHold, ControlTransitionLike, ControlTransitionPress,
    ControlTransitionRelease,
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

type CharacterControlTransitionsTransitionSystemData<'s> = (
    Read<'s, EventChannel<ControlInputEvent>>,
    CharacterControlTransitionsTransitionResources<'s>,
    ControlTransitionRequirementSystemData<'s>,
);

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
                    let control_transition_requirement =
                        character_control_transition.control_transition_requirement;

                    match control_transition {
                        ControlTransition::Press(ControlTransitionPress {
                            action,
                            sequence_id,
                        }) => {
                            if value && control_action == action {
                                Some((sequence_id, control_transition_requirement))
                            } else {
                                None
                            }
                        }
                        ControlTransition::Release(ControlTransitionRelease {
                            action,
                            sequence_id,
                        }) => {
                            if !value && control_action == action {
                                Some((sequence_id, control_transition_requirement))
                            } else {
                                None
                            }
                        }
                        ControlTransition::Hold(control_transition_hold) => {
                            Self::hold_transition(control_transition_hold, *controller_input)
                                .map(|transition| (transition, control_transition_requirement))
                        }
                    }
                })
                .filter_map(|(sequence_id, control_transition_requirement)| {
                    if let Some(control_transition_requirement) = control_transition_requirement {
                        if Self::transition_requirement_met(
                            control_transition_requirement_system_data,
                            control_transition_requirement,
                            entity,
                        ) {
                            Some(sequence_id)
                        } else {
                            None
                        }
                    } else {
                        Some(sequence_id)
                    }
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
    /// Checks the `ControllerInput` state for any `Hold` transitions.
    fn process_control_transition_holds(
        &self,
        CharacterControlTransitionsTransitionResources {
            ref entities,
            ref controller_inputs,
            ref character_control_transitions_handles,
            ref character_control_transitions_assets,
            ref mut character_sequence_ids,
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
                            let control_transition_requirement =
                                character_control_transition.control_transition_requirement;

                            if let ControlTransition::Hold(control_transition_hold) =
                                control_transition
                            {
                                Self::hold_transition(*control_transition_hold, *controller_input)
                                    .map(|transition| (transition, control_transition_requirement))
                            } else {
                                None
                            }
                        })
                        .filter_map(|(sequence_id, control_transition_requirement)| {
                            if let Some(control_transition_requirement) =
                                control_transition_requirement
                            {
                                if Self::transition_requirement_met(
                                    control_transition_requirement_system_data,
                                    control_transition_requirement,
                                    entity,
                                ) {
                                    Some(sequence_id)
                                } else {
                                    None
                                }
                            } else {
                                Some(sequence_id)
                            }
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

    /// Returns the transition sequence ID if the button for that hold transition is held.
    ///
    /// # Parameters
    ///
    /// * `control_transition_hold`: `ControlAction` and sequence ID the hold transition applies to.
    /// * `controller_input`: Controller input status.
    fn hold_transition(
        ControlTransitionHold {
            action,
            sequence_id,
        }: ControlTransitionHold<CharacterSequenceId>,
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

    fn transition_requirement_met(
        ControlTransitionRequirementSystemData {
            health_pointses: _health_pointses,
            skill_pointses: _skill_pointses,
            charge_pointses: _charge_pointses,
        }: &ControlTransitionRequirementSystemData,
        _control_transition_requirement: ControlTransitionRequirement,
        _entity: Entity,
    ) -> bool {
        // TODO: Check if character meets requirement.
        true
    }
}

impl<'s> System<'s> for CharacterControlTransitionsTransitionSystem {
    type SystemData = CharacterControlTransitionsTransitionSystemData<'s>;

    fn run(
        &mut self,
        (
            control_input_ec,
            mut character_control_transitions_transition_resources,
            control_transition_requirement_system_data,
        ): Self::SystemData,
    ) {
        self.processed_entities.clear();

        let control_input_event_rid = self
            .control_input_event_rid
            .as_mut()
            .expect("Expected `control_input_event_rid` field to be set.");

        control_input_ec
            .read(control_input_event_rid)
            .for_each(|ev| {
                if let ControlInputEvent::ControlActionPressed(control_action_event_data) = ev {
                    self.handle_event(
                        &mut character_control_transitions_transition_resources,
                        &control_transition_requirement_system_data,
                        *control_action_event_data,
                        true,
                    );
                } else if let ControlInputEvent::ControlActionReleased(control_action_event_data) =
                    ev
                {
                    self.handle_event(
                        &mut character_control_transitions_transition_resources,
                        &control_transition_requirement_system_data,
                        *control_action_event_data,
                        false,
                    );
                }
            });

        self.process_control_transition_holds(
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
    use amethyst::{
        ecs::{Entity, World, WriteStorage},
        shrev::EventChannel,
        Error,
    };
    use application_test_support::{AutexousiousApplication, ObjectQueries, SequenceQueries};
    use assets_test::CHAR_BAT_SLUG;
    use character_model::{config::CharacterSequenceId, loaded::CharacterControlTransitionsHandle};
    use game_input::ControllerInput;
    use game_input_model::{ControlAction, ControlActionEventData, ControlInputEvent};
    use object_type::ObjectType;

    use super::CharacterControlTransitionsTransitionSystem;

    #[test]
    fn inserts_transition_for_press_event() -> Result<(), Error> {
        run_test(
            CharacterSequenceId::Stand,
            None,
            Some(|entity| {
                let control_action_event_data = ControlActionEventData {
                    entity,
                    control_action: ControlAction::Attack,
                };
                ControlInputEvent::ControlActionPressed(control_action_event_data)
            }),
            CharacterSequenceId::StandAttack0,
        )
    }

    #[test]
    fn inserts_transition_for_release_event() -> Result<(), Error> {
        run_test(
            CharacterSequenceId::Stand,
            None,
            Some(|entity| {
                let control_action_event_data = ControlActionEventData {
                    entity,
                    control_action: ControlAction::Special,
                };
                ControlInputEvent::ControlActionReleased(control_action_event_data)
            }),
            CharacterSequenceId::DashBack,
        )
    }

    #[test]
    fn inserts_transition_for_hold() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.defend = true;

        run_test(
            CharacterSequenceId::Stand,
            Some(controller_input),
            None,
            CharacterSequenceId::Flinch0,
        )
    }

    #[test]
    fn prioritizes_press_over_hold_transition() -> Result<(), Error> {
        let mut controller_input = ControllerInput::default();
        controller_input.jump = true;

        run_test(
            CharacterSequenceId::Stand,
            Some(controller_input),
            None,
            CharacterSequenceId::DashForward,
        )
    }

    #[test]
    fn prioritizes_release_over_hold_transition() -> Result<(), Error> {
        // hold `Defend` but release `Special`
        let mut controller_input = ControllerInput::default();
        controller_input.defend = true;

        run_test(
            CharacterSequenceId::Stand,
            Some(controller_input),
            Some(|entity| {
                let control_action_event_data = ControlActionEventData {
                    entity,
                    control_action: ControlAction::Special,
                };
                ControlInputEvent::ControlActionReleased(control_action_event_data)
            }),
            CharacterSequenceId::DashBack,
        )
    }

    fn run_test(
        setup_sequence_id: CharacterSequenceId,
        setup_controller_input: Option<ControllerInput>,
        control_input_event_fn: Option<fn(Entity) -> ControlInputEvent>,
        expected_sequence_id: CharacterSequenceId,
    ) -> Result<(), Error> {
        AutexousiousApplication::game_base()
            .with_system(CharacterControlTransitionsTransitionSystem::new(), "", &[])
            .with_setup(move |world| {
                let entity = ObjectQueries::game_object_entity(world, ObjectType::Character);
                let character_control_transitions_handle =
                    SequenceQueries::character_control_transitions_handle(
                        world,
                        &CHAR_BAT_SLUG.clone(),
                        CharacterSequenceId::Stand,
                        0,
                    );
                {
                    let (
                        mut character_sequence_ids,
                        mut character_control_transitions_handles,
                        mut controller_inputs,
                    ) = world.system_data::<(
                        WriteStorage<'_, CharacterSequenceId>,
                        WriteStorage<'_, CharacterControlTransitionsHandle>,
                        WriteStorage<'_, ControllerInput>,
                    )>();

                    character_sequence_ids
                        .insert(entity, setup_sequence_id)
                        .expect("Failed to insert `CharacterSequenceId` component.");
                    character_control_transitions_handles
                        .insert(entity, character_control_transitions_handle)
                        .expect("Failed to insert `CharacterControlTransitionsHandle` component.");

                    if let Some(setup_controller_input) = setup_controller_input {
                        controller_inputs
                            .insert(entity, setup_controller_input)
                            .expect("Failed to insert `ControllerInput` component.");
                    }
                }

                if let Some(control_input_event_fn) = control_input_event_fn {
                    send_event(world, control_input_event_fn(entity));
                }

                world.add_resource(entity);
            })
            .with_assertion(move |world| {
                let entity = *world.read_resource::<Entity>();

                let character_sequence_ids = world.read_storage::<CharacterSequenceId>();
                let character_sequence_id = character_sequence_ids
                    .get(entity)
                    .expect("Expected `CharacterSequenceId` component to exist.");

                assert_eq!(&expected_sequence_id, character_sequence_id);
            })
            .run_isolated()
    }

    fn send_event(world: &mut World, event: ControlInputEvent) {
        let mut ec = world.write_resource::<EventChannel<ControlInputEvent>>();
        ec.single_write(event);
    } // kcov-ignore
}
