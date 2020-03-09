use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, World, Write},
    input::InputEvent,
    shred::{ResourceId, SystemData},
    shrev::{EventChannel, ReaderId},
};
use derivative::Derivative;
use derive_new::new;
use game_input_model::{
    config::{ControlBindings, PlayerActionControl, PlayerAxisControl},
    play::{
        AxisMoveEventData, ControlActionEventData, ControlInputEvent, InputControlled,
        SharedInputControlled,
    },
};

/// Sends `ControlInputEvent`s based on the `InputHandler` state.
#[derive(Debug, SystemDesc, new)]
#[system_desc(name(InputToControlInputSystemDesc))]
pub struct InputToControlInputSystem {
    /// Reader ID for the `InputEvent` channel.
    #[system_desc(event_channel_reader)]
    input_event_rid: ReaderId<InputEvent<ControlBindings>>,
    /// Pre-allocated vector
    #[new(value = "Vec::with_capacity(64)")]
    #[system_desc(skip)]
    control_input_events: Vec<ControlInputEvent>,
}

#[derive(Derivative, SystemData)]
#[derivative(Debug)]
pub struct InputToControlInputSystemData<'s> {
    /// `InputEvent<ControlBindings>` channel.
    #[derivative(Debug = "ignore")]
    pub input_ec: Read<'s, EventChannel<InputEvent<ControlBindings>>>,
    /// `Entities` resource.
    #[derivative(Debug = "ignore")]
    pub entities: Entities<'s>,
    /// `InputControlled` components.
    #[derivative(Debug = "ignore")]
    pub input_controlleds: ReadStorage<'s, InputControlled>,
    /// `SharedInputControlled` components.
    #[derivative(Debug = "ignore")]
    pub shared_input_controlleds: ReadStorage<'s, SharedInputControlled>,
    /// `ControlInputEvent` channel.
    #[derivative(Debug = "ignore")]
    pub control_input_ec: Write<'s, EventChannel<ControlInputEvent>>,
}

impl<'s> System<'s> for InputToControlInputSystem {
    type SystemData = InputToControlInputSystemData<'s>;

    fn run(
        &mut self,
        InputToControlInputSystemData {
            input_ec,
            entities,
            input_controlleds,
            shared_input_controlleds,
            mut control_input_ec,
        }: Self::SystemData,
    ) {
        input_ec.read(&mut self.input_event_rid).for_each(|ev| {
            match *ev {
                InputEvent::ActionPressed(PlayerActionControl { player, action }) => {
                    // Find the entity has the `player` control id in its `InputControlled`
                    // component.

                    let shared_input_controlled_entities = (&entities, &shared_input_controlleds)
                        .join()
                        .map(|(entity, _)| entity);

                    let control_input_events_iter = (&entities, &input_controlleds)
                        .join()
                        .filter_map(|(entity, input_controlled)| {
                            if input_controlled.controller_id == player {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .chain(shared_input_controlled_entities)
                        .map(|entity| {
                            ControlInputEvent::ControlActionPress(ControlActionEventData {
                                controller_id: player,
                                entity,
                                control_action: action,
                            })
                        });

                    self.control_input_events.extend(control_input_events_iter);
                }
                InputEvent::ActionReleased(PlayerActionControl { player, action }) => {
                    let shared_input_controlled_entities = (&entities, &shared_input_controlleds)
                        .join()
                        .map(|(entity, _)| entity);

                    let control_input_events_iter = (&entities, &input_controlleds)
                        .join()
                        .filter_map(|(entity, input_controlled)| {
                            if input_controlled.controller_id == player {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .chain(shared_input_controlled_entities)
                        .map(|entity| {
                            ControlInputEvent::ControlActionRelease(ControlActionEventData {
                                controller_id: player,
                                entity,
                                control_action: action,
                            })
                        });

                    self.control_input_events.extend(control_input_events_iter);
                }
                InputEvent::AxisMoved {
                    axis: PlayerAxisControl { player, axis },
                    value,
                } => {
                    let shared_input_controlled_entities = (&entities, &shared_input_controlleds)
                        .join()
                        .map(|(entity, _)| entity);

                    let control_input_events_iter = (&entities, &input_controlleds)
                        .join()
                        .filter_map(|(entity, input_controlled)| {
                            if input_controlled.controller_id == player {
                                Some(entity)
                            } else {
                                None
                            }
                        })
                        .chain(shared_input_controlled_entities)
                        .map(|entity| {
                            ControlInputEvent::AxisMoved(AxisMoveEventData {
                                controller_id: player,
                                entity,
                                axis,
                                value,
                            })
                        });

                    self.control_input_events.extend(control_input_events_iter);
                }
                _ => {}
            }
        });

        control_input_ec.drain_vec_write(&mut self.control_input_events);
    }
}
